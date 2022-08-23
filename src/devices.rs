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

use std::collections::HashMap;

use rumqttc::QoS;

use crate::mqtt::{ActionMessage, ConnectionMessage};

pub fn simulate_relay(
    roottopic: &str,
) -> impl Fn(&mut HashMap<String, Vec<u8>>, &ActionMessage) -> Vec<ConnectionMessage> {
    let root_topic = String::from(roottopic);
    let key_topic = format!("simulate_relay_{}", roottopic);
    let action_topic = format!("{}/set", roottopic);
    move |mapinfo: &mut HashMap<String, Vec<u8>>,
          action: &ActionMessage|
          -> Vec<ConnectionMessage> {
        if action.matches(&action_topic) {
            let value = String::from_utf8_lossy(&action.payload);

            let newvalue = if value == "1" || value == "on" {
                "1"
            } else if value == "0" || value == "off" {
                "0"
            } else if value == "2" || value == "toggle" {
                let current = mapinfo
                    .get(&key_topic)
                    .map(|s| String::from_utf8_lossy(s))
                    .unwrap_or_default();
                if current == "1" {
                    "0"
                } else {
                    "1"
                }
            } else {
                ""
            }
            .as_bytes();
            mapinfo.insert(key_topic.clone(), newvalue.to_vec());
            return vec![ConnectionMessage {
                topic: root_topic.clone(),
                payload: newvalue.into(),
                qos: QoS::AtMostOnce,
                retain: true,
            }];
        }

        vec![]
    }
}
