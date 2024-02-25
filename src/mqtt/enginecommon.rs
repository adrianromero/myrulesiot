//    MyRulesIoT is a rules engine for MQTT
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

use rumqttc::{Publish, QoS};

#[derive(Debug)]
pub struct EngineAction {
    pub topic: String,
    pub payload: Vec<u8>,
    pub timestamp: i64,
}

impl EngineAction {
    pub fn matches(&self, filter: &str) -> bool {
        rumqttc::matches(&self.topic, filter)
    }
    pub fn matches_action(&self, filter: &str, payload: Vec<u8>) -> bool {
        rumqttc::matches(&self.topic, filter) && payload.eq(&self.payload)
    }
}

impl From<Publish> for EngineAction {
    fn from(p: Publish) -> EngineAction {
        EngineAction {
            topic: p.topic,
            payload: p.payload.into(),
            timestamp: chrono::Local::now().timestamp_millis(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EngineMessage {
    pub qos: QoS,
    pub retain: bool,
    pub topic: String,
    pub payload: Vec<u8>,
}

#[derive(Debug)]
pub struct EngineResult {
    pub messages: Vec<EngineMessage>,
    pub is_final: bool,
}
