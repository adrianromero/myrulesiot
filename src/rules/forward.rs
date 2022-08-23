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
use serde_json::json;
use serde_json::Value;

use crate::mqtt::{ActionMessage, ConnectionMessage};

pub fn forward_user_action_tick(
    strtopic: &str,
) -> impl Fn(&mut HashMap<String, Vec<u8>>, &ActionMessage) -> Vec<ConnectionMessage> {
    let topic = strtopic.to_string();
    move |_: &mut HashMap<String, Vec<u8>>, action: &ActionMessage| -> Vec<ConnectionMessage> {
        if action.matches("SYSMR/user_action/tick") {
            return vec![ConnectionMessage {
                topic: topic.clone(),
                payload: action.payload.clone(),
                qos: QoS::AtMostOnce,
                retain: false,
            }];
        }
        vec![]
    }
}

pub fn forward_action(
    stractiontopic: &str,
    strtopic: &str,
) -> impl Fn(&mut HashMap<String, Vec<u8>>, &ActionMessage) -> Vec<ConnectionMessage> {
    let topic_value: String = String::from(strtopic);
    let action_topic_value: String = String::from(stractiontopic);

    move |mapinfo: &mut HashMap<String, Vec<u8>>,
          action: &ActionMessage|
          -> Vec<ConnectionMessage> {
        if action.matches(&action_topic_value) {
            let json_payload: Value =
                serde_json::from_slice(&action.payload).unwrap_or(json!(null));
            if json_payload["action"] == json!("toggle") {
                let status = mapinfo
                    .get(&topic_value)
                    .map(|s| String::from_utf8_lossy(&s));

                let newvalue: Vec<u8> = match status {
                    None => vec![1],
                    Some(st) => {
                        if st == "1" {
                            vec![0]
                        } else {
                            vec![1]
                        }
                    }
                }
                .into();

                mapinfo.insert(topic_value.clone(), newvalue.to_vec());
                return vec![ConnectionMessage {
                    topic: topic_value.clone(),
                    payload: newvalue,
                    qos: QoS::AtMostOnce,
                    retain: false,
                }];
            }
        }
        return vec![];
    }
}
