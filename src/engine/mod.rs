//    MyRulesIoT is a rules engine for MQTT
//    Copyright (C) 2021 Adrián Romero Corchado.
//
//    This file is part of MyRulesIoT.
//
//    MyRulesIoT is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    MyRulesIoT is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with MyRulesIoT.  If not, see <http://www.gnu.org/licenses/>.
//

use std::fmt::Debug;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::task;

pub struct Engine<A, R, S> {
    pub reduce: fn(S, A) -> S,
    pub template: fn(&S) -> R,
    pub is_final: fn(&R) -> bool,
}

pub async fn runtime_loop<A, R, S>(
    tx: broadcast::Sender<R>,
    mut rx: mpsc::Receiver<A>,
    engine: Engine<A, R, S>,
) -> Result<(), broadcast::error::SendError<R>>
where
    A: Debug,
    R: Debug,
    S: Default + Debug,
{
    let mut state = Default::default();

    while let Some(action) = rx.recv().await {
        log::info!("Persist action {:?}.", &action);

        state = (engine.reduce)(state, action);

        log::info!("Persist state {:?}.", &state);

        let result = (engine.template)(&state);

        log::info!("Persist result {:?}.", &result);

        let is_final = (engine.is_final)(&result);

        tx.send(result)?;

        if is_final {
            break;
        }
    }
    Ok(())
}

pub fn task_runtime_loop<A, R, S>(
    tx: broadcast::Sender<R>,
    rx: mpsc::Receiver<A>,
    engine: Engine<A, R, S>,
) -> task::JoinHandle<()>
where
    A: Send + Debug + 'static,
    R: Send + Debug + 'static,
    S: Send + Default + Debug + 'static,
{
    task::spawn(async move {
        match runtime_loop(tx, rx, engine).await {
            Result::Ok(_) => {}
            Result::Err(error) => {
                log::warn!("Runtime error {}", error);
            }
        }
        log::info!("Exiting spawn runtime engine...");
    })
}
