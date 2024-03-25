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

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
pub struct EngineAction {
    pub topic: String,
    pub payload: Vec<u8>,
}

impl EngineAction {
    pub fn new(topic: String, payload: Vec<u8>) -> Self {
        EngineAction { topic, payload }
    }
    pub fn matches(&self, filter: &str) -> bool {
        self.topic.eq(filter)
    }
    pub fn matches_action(&self, filter: &str, payload: &[u8]) -> bool {
        self.topic.eq(filter) && payload.eq(&self.payload)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineMessage {
    pub topic: String,
    pub payload: Vec<u8>,
    pub properties: Value,
}

impl EngineMessage {
    pub fn new(topic: String, payload: Vec<u8>) -> Self {
        EngineMessage {
            topic,
            payload,
            properties: Value::Null,
        }
    }

    pub fn new_json(topic: String, payload: Value) -> Self {
        EngineMessage {
            topic,
            payload: payload.to_string().into_bytes(),
            properties: Value::Null,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EngineResult {
    pub messages: Vec<EngineMessage>,
    pub is_final: bool,
}
