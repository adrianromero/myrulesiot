//    MyRulesIoT is a rules engine library for MQTT
//    Copyright (C) 2024 Adrián Romero Corchado.
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
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task;

pub struct MultiRX<T> {
    rx: Receiver<T>,
    txs: Vec<Sender<T>>,
}

impl<T> MultiRX<T>
where
    T: Clone + Send + 'static,
{
    pub fn new(rx: Receiver<T>) -> Self {
        MultiRX { rx, txs: vec![] }
    }

    pub fn create(&mut self) -> Receiver<T> {
        let (tx, rx) = mpsc::channel::<T>(10);
        self.txs.push(tx);
        rx
    }

    pub fn task_publication_loop(mut self) -> task::JoinHandle<()> {
        task::spawn(async move {
            log::debug!("Started MultiSubscriber loop...");

            while let Some(v) = self.rx.recv().await {
                for tx in self.txs.iter() {
                    if let Err(error) = tx.send(v.clone()).await {
                        log::warn!("Exited MultiSubscriber loop with error {}", error);
                        return;
                    }
                }
            }

            log::debug!("Exited MultiSubscriber loop...");
        })
    }
}
