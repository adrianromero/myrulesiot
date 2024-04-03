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

use tokio::sync::mpsc;
use tokio::time;

use super::EngineAction;

pub async fn task_timer_loop(tx: mpsc::Sender<EngineAction>, duration: chrono::Duration) {
    let time_duration = time::Duration::from_millis(duration.num_milliseconds() as u64);

    log::debug!("Starting user action tick subscription...");
    loop {
        time::sleep(time_duration).await;
        if tx
            .send(EngineAction::new(
                "SYSMR/action/tick".to_string(),
                chrono::Local::now().to_rfc3339().into_bytes(),
            ))
            .await
            .is_err()
        {
            // If cannot send because channel closed, just ignore and exit.
            break;
        }
    }
    log::debug!("Exiting user action tick subscription...");
}
