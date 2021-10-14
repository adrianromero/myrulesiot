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
pub struct ConnectionMessage {
    pub qos: QoS,
    pub retain: bool,
    pub topic: String,
    pub payload: Bytes,
}

impl Default for ConnectionMessage {
    fn default() -> Self {
        ConnectionMessage {
            topic: "".into(),
            payload: Bytes::new(),
            qos: QoS::AtLeastOnce,
            retain: false,
        }
    }
}

impl From<Publish> for ConnectionMessage {
    fn from(p: Publish) -> ConnectionMessage {
        ConnectionMessage {
            topic: p.topic,
            retain: p.retain,
            qos: p.qos,
            payload: p.payload,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionResult {
    pub messages: Vec<ConnectionMessage>,
    pub is_final: bool,
}
