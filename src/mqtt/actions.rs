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

use bytes::Bytes;
use rumqttc::Publish;
use rumqttc::QoS;

#[derive(Debug, Clone)]
pub struct ActionMessage {
    pub topic: String,
    pub payload: Bytes,
    pub timestamp: i64,
}

impl ActionMessage {
    pub fn matches(&self, filter: &str) -> bool {
        rumqttc::matches(&self.topic, filter)
    }
    pub fn matches_action(&self, filter: &str, payload: Bytes) -> bool {
        rumqttc::matches(&self.topic, filter) && payload.eq(&self.payload)
    }
}

impl From<Publish> for ActionMessage {
    fn from(p: Publish) -> ActionMessage {
        ActionMessage {
            topic: p.topic,
            payload: p.payload,
            timestamp: chrono::Local::now().timestamp_millis(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionMessage {
    pub qos: QoS,
    pub retain: bool,
    pub topic: String,
    pub payload: Bytes,
}

#[derive(Debug, Clone)]
pub struct ConnectionResult {
    pub messages: Vec<ConnectionMessage>,
    pub is_final: bool,
}
