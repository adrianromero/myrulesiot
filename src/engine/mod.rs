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
use tokio::sync::mpsc;

pub struct Engine<A, R, S> {
    pub reduce: fn(&S, &A) -> S,
    pub template: fn(&S) -> R,
    pub is_final: fn(&R) -> bool,
}

pub struct RuntimeEngine;

impl RuntimeEngine {
    pub async fn do_loop<A, R, S>(
        engine: Engine<A, R, S>,
        tx: mpsc::Sender<R>,
        mut rx: mpsc::Receiver<A>,
    ) -> Result<(), mpsc::error::SendError<R>>
    where
        A: Debug,
        R: Debug,
        S: Default + Debug,
    {
        let mut state = Default::default();

        while let Some(action) = rx.recv().await {
            log::info!("Persist action {:?}.", &action);

            state = (engine.reduce)(&state, &action);

            log::info!("Persist state {:?}.", &state);

            let result = (engine.template)(&state);

            log::info!("Persist result {:?}.", &result);

            let is_final = (engine.is_final)(&result);

            tx.send(result).await?;

            if is_final {
                break;
            }
        }
        Ok(())
    }
}
