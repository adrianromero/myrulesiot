//    MyRulesIoT is a rules engine for MQTT
//    Copyright (C) 2021-2024 Adrián Romero Corchado.
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

use std::fs;
use tokio::sync::mpsc;
use tokio::task;

use super::EngineAction;

pub fn task_file_functions_loop(
    tx: &mpsc::Sender<EngineAction>,
    prefix_id: &str,
    path: &str,
) -> task::JoinHandle<()> {
    let task_tx = tx.clone();
    let prefix_id = String::from(prefix_id);
    let functions = fs::read(path);

    task::spawn(async move {
        log::debug!("Started file functions load...");

        task_tx
            .send(EngineAction::new(
                format!("{}/command/functions_putall", prefix_id),
                functions.unwrap(),
            ))
            .await
            .unwrap();

        log::debug!("Exited file functions load...");
    })
}
