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

use std::time::Duration;
// use job_scheduler::{Job, JobScheduler};
// use chrono::Duration;

use tokio::sync::mpsc;
use tokio::task;
use tokio::time;

use crate::mqtt::ActionMessage;

// fn job_scheduler() {
//     let mut sched = JobScheduler::new();
//     sched.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
//         println!("I get executed every 10 seconds!");
//     }));
//     loop {
//         sched.tick();
//         std::thread::sleep(Duration::from_millis(500));
//     }
// }

pub fn task_timer_loop(tx: &mpsc::Sender<ActionMessage>, duration: u64) -> task::JoinHandle<()> {
    let timer_tx = tx.clone();
    // let expression = "0   30   9,12,15     1,15       May-Aug  Mon,Wed,Fri  2018/2";
    // let schedule = Schedule::from_str(expression).unwrap();

    task::spawn(async move {
        log::debug!("Started timer tick subscription...");
        loop {
            time::sleep(Duration::from_millis(duration)).await;
            if timer_tx
                .send(ActionMessage {
                    topic: "SYSMR/user_action".into(),
                    payload: "tick".into(),
                    ..ActionMessage::default()
                })
                .await
                .is_err()
            {
                // If cannot send because channel closed, just ignore and exit.
                break;
            }
        }
        log::debug!("Exited timer tick subscription...");
    })
}
