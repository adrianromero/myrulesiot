//    MyRulesIoT is a rules engine library for MQTT
//    Copyright (C) 2021-2022 Adrián Romero Corchado.
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
use tokio::sync::mpsc;
use tokio::task;

pub struct Engine<A, R, S>
where
    A: Debug,
    R: Debug,
    S: Debug + Default,
{
    pub reduce: Box<dyn Fn(S, A) -> S + Send + 'static>,
    pub template: Box<dyn Fn(&S) -> R + Send + 'static>,
    pub is_final: Box<dyn Fn(&R) -> bool + Send + 'static>,
}

pub async fn runtime_loop<A, R, S>(
    tx: mpsc::Sender<R>,
    mut rx: mpsc::Receiver<A>,
    engine: Engine<A, R, S>,
) -> Result<(), mpsc::error::SendError<R>>
where
    A: Debug,
    R: Debug,
    S: Debug + Default,
{
    let mut state = Default::default();

    while let Some(action) = rx.recv().await {
        log::debug!("Persist action {:?}.", &action);

        state = (engine.reduce)(state, action);

        log::debug!("Persist state {:?}.", &state);

        let result = (engine.template)(&state);

        log::debug!("Persist result {:?}.", &result);

        let is_final = (engine.is_final)(&result);

        tx.send(result).await?;

        if is_final {
            break;
        }
    }
    Ok(())
}

pub fn task_runtime_loop<A, R, S>(
    tx: mpsc::Sender<R>,
    rx: mpsc::Receiver<A>,
    engine: Engine<A, R, S>,
) -> task::JoinHandle<()>
where
    A: Debug + Send + 'static,
    R: Debug + Send + 'static,
    S: Debug + Default + Send + 'static,
{
    task::spawn(async move {
        log::info!("Started runtime engine...");
        match runtime_loop(tx, rx, engine).await {
            Result::Ok(_) => {}
            Result::Err(error) => {
                log::warn!("Runtime error {}", error);
            }
        }
        log::info!("Exited runtime engine...");
    })
}
