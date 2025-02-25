//    MyRulesIoT is a rules engine for MQTT
//    Copyright (C) 2021-2025 Adrián Romero Corchado.
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

use tokio::sync::broadcast;
use tokio::sync::mpsc;

use super::{EngineAction, EngineResult};

pub async fn task_load_functions_loop(tx: mpsc::Sender<EngineAction>, functions: Vec<u8>) {
    log::debug!("Starting file functions load...");

    tx.send(EngineAction::new(
        "SYSMR/action/load_functions".into(),
        functions,
    ))
    .await
    .unwrap();

    log::debug!("Exiting file functions load...");
}

pub async fn task_save_functions_loop(
    mut rx: broadcast::Receiver<EngineResult>,
) -> Option<Vec<u8>> {
    let mut functions: Option<Vec<u8>> = None;
    log::debug!("Starting file functions save...");
    while let Ok(res) = rx.recv().await {
        for elem in res.messages.into_iter() {
            if elem.topic.eq("SYSMR/notify/save_functions") {
                functions = Some(elem.payload);
            }
        }
    }
    log::debug!("Exiting file functions save...");
    functions
}

pub async fn task_save_exit_loop(
    prefix_id: String,
    mut rx: broadcast::Receiver<EngineResult>,
) -> Option<Vec<u8>> {
    let topic = format!("{}/notify/exit", prefix_id);
    let mut exit: Option<Vec<u8>> = None;
    log::debug!("Starting file exit save...");
    while let Ok(res) = rx.recv().await {
        for elem in res.messages.into_iter() {
            if elem.topic.eq(&topic) {
                exit = Some(elem.payload);
            }
        }
    }
    log::debug!("Exiting file exit save...");
    exit
}
