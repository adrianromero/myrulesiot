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

use std::collections::HashMap;

use rumqttc::QoS;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

use crate::mqtt::{EngineAction, EngineMessage};

#[derive(Serialize, Deserialize)]
struct ForwardUserActionParam {
    topic: String,
    forwardtopic: String,
}

pub fn engine_forward_user_action(
    mapinfo: &mut HashMap<String, Vec<u8>>,
    action: &EngineAction,
    params: &serde_json::Value,
) -> Vec<EngineMessage> {
    let p: ForwardUserActionParam = serde_json::from_value(params.clone()).unwrap();
    forward_user_action(mapinfo, action, &p.topic, &p.forwardtopic)
}

fn forward_user_action(
    _: &mut HashMap<String, Vec<u8>>,
    action: &EngineAction,
    topic: &str,
    forwardtopic: &str,
) -> Vec<EngineMessage> {
    if action.matches(topic) {
        return vec![EngineMessage {
            topic: String::from(forwardtopic),
            payload: action.payload.clone(),
            qos: QoS::AtMostOnce,
            retain: false,
        }];
    }
    vec![]
}

#[derive(Serialize, Deserialize)]
struct ForwardActionParam {
    topic: String,
    forwardtopic: String,
}

pub fn engine_forward_action(
    mapinfo: &mut HashMap<String, Vec<u8>>,
    action: &EngineAction,
    params: &serde_json::Value,
) -> Vec<EngineMessage> {
    let p: ForwardActionParam = serde_json::from_value(params.clone()).unwrap();
    forward_action(mapinfo, action, &p.topic, &p.forwardtopic)
}

fn forward_action(
    mapinfo: &mut HashMap<String, Vec<u8>>,
    action: &EngineAction,
    topic: &str,
    forwardtopic: &str,
) -> Vec<EngineMessage> {
    if action.matches(topic) {
        let json_payload: Value = serde_json::from_slice(&action.payload).unwrap_or(json!(null));
        if json_payload["action"] == json!("toggle") {
            let status = mapinfo
                .get(forwardtopic)
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
            };

            mapinfo.insert(String::from(forwardtopic), newvalue.clone());
            return vec![EngineMessage {
                topic: String::from(forwardtopic),
                payload: newvalue,
                qos: QoS::AtMostOnce,
                retain: false,
            }];
        }
    }
    return vec![];
}
