//    MyRulesIoT  Project is a rules engine for MQTT based on MyRulesIoT lib
//    Copyright (C) 2024  Adrián Romero Corchado.
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

use crate::master::{EngineAction, EngineResult, EngineState, MasterEngine};
use crate::rules;
use crate::runtime;

use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct RuntimeTester {
    opt_sub_tx: Option<Sender<EngineAction>>,
    opt_sub_rx: Option<Receiver<EngineAction>>,
    opt_pub_tx: Option<Sender<EngineResult>>,
    pub_rx: Receiver<EngineResult>,
}

impl RuntimeTester {
    pub fn new() -> Self {
        let (sub_tx, sub_rx) = mpsc::channel::<EngineAction>(10);
        let (pub_tx, pub_rx) = mpsc::channel::<EngineResult>(10);
        RuntimeTester {
            opt_sub_tx: Some(sub_tx),
            opt_sub_rx: Some(sub_rx),
            opt_pub_tx: Some(pub_tx),
            pub_rx,
        }
    }

    pub async fn send(&self, action: EngineAction) {
        self.opt_sub_tx
            .as_ref()
            .unwrap()
            .send(action)
            .await
            .unwrap();
    }

    pub async fn recv(&mut self) -> Option<EngineResult> {
        self.pub_rx.recv().await
    }

    pub async fn runtime_loop(&mut self) {
        let engine_functions = rules::default_engine_functions();
        runtime::task_runtime_loop(
            self.opt_pub_tx.as_ref().unwrap().clone(),
            self.opt_sub_rx.take().unwrap(),
            MasterEngine::new(String::from("MYRULESTEST"), engine_functions),
            EngineState::default(),
        )
        .await;

        self.opt_pub_tx.take();
        self.opt_sub_tx.take();
    }
}
