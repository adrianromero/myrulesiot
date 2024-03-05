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

use rumqttc::QoS;
use serde_json::json;
use serde_json::Value;

use crate::mqtt::{EngineAction, EngineMessage};

// #[derive(Serialize, Deserialize)]
// struct ForwardUserActionParam {
//     topic: String,
//     forwardtopic: String,
// }

pub fn engine_forward_user_action(
    loopstack: &mut serde_json::Value,
    mapinfo: &mut serde_json::Value,
    action: &EngineAction,
    params: &serde_json::Value,
) -> Vec<EngineMessage> {
    let topic = params["topic"].as_str().unwrap();
    let forwardtopic = params["forwardtopic"].as_str().unwrap();
    forward_user_action(loopstack, mapinfo, action, topic, forwardtopic)
}

fn forward_user_action(
    _loopstack: &mut serde_json::Value,
    _info: &mut serde_json::Value,
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

pub fn engine_forward_action(
    loopstack: &mut serde_json::Value,
    mapinfo: &mut serde_json::Value,
    action: &EngineAction,
    params: &serde_json::Value,
) -> Vec<EngineMessage> {
    let topic = params["topic"].as_str().unwrap();
    let forwardtopic = params["forwardtopic"].as_str().unwrap();
    forward_action(loopstack, mapinfo, action, topic, forwardtopic)
}

fn forward_action(
    _loopstack: &mut serde_json::Value,
    mapinfo: &mut serde_json::Value,
    action: &EngineAction,
    topic: &str,
    forwardtopic: &str,
) -> Vec<EngineMessage> {
    if action.matches(topic) {
        let json_payload: Value = serde_json::from_slice(&action.payload).unwrap_or(json!(null));
        if json_payload["action"] == json!("toggle") {
            let status = mapinfo[forwardtopic].as_bool();
            let newvalue: bool = match status {
                None => true,
                Some(st) => !st,
            };
            mapinfo[forwardtopic] = json!(newvalue);
            return vec![EngineMessage {
                topic: String::from(forwardtopic),
                payload: if newvalue { vec![1] } else { vec![0] },
                qos: QoS::AtMostOnce,
                retain: false,
            }];
        }
    }
    return vec![];
}
