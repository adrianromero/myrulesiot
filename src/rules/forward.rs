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
use serde_json::json;
use serde_json::Value;

use crate::mqtt::{EngineAction, EngineMessage};

pub fn engine_forward_user_action_tick(
    mapinfo: &mut HashMap<String, Vec<u8>>,
    action: &EngineAction,
    params: &[String],
) -> Vec<EngineMessage> {
    forward_user_action_tick(mapinfo, action, &params[0])
}

pub fn box_forward_user_action_tick(
    topic: &str,
) -> impl Fn(&mut HashMap<String, Vec<u8>>, &EngineAction) -> Vec<EngineMessage> {
    let topic: String = String::from(topic);
    move |mapinfo: &mut HashMap<String, Vec<u8>>, action: &EngineAction| -> Vec<EngineMessage> {
        forward_user_action_tick(mapinfo, action, &topic)
    }
}

fn forward_user_action_tick(
    _: &mut HashMap<String, Vec<u8>>,
    action: &EngineAction,
    topic: &str,
) -> Vec<EngineMessage> {
    if action.matches("SYSMR/user_action/tick") {
        return vec![EngineMessage {
            topic: String::from(topic),
            payload: action.payload.clone(),
            qos: QoS::AtMostOnce,
            retain: false,
        }];
    }
    vec![]
}

pub fn box_forward_action(
    topic: &str,
    forwardtopic: &str,
) -> impl Fn(&mut HashMap<String, Vec<u8>>, &EngineAction) -> Vec<EngineMessage> {
    let topic: String = String::from(topic);
    let forwardtopic: String = String::from(forwardtopic);
    move |mapinfo: &mut HashMap<String, Vec<u8>>, action: &EngineAction| -> Vec<EngineMessage> {
        forward_action(mapinfo, action, &topic, &forwardtopic)
    }
}

pub fn engine_forward_action(
    mapinfo: &mut HashMap<String, Vec<u8>>,
    action: &EngineAction,
    params: &[String],
) -> Vec<EngineMessage> {
    forward_action(mapinfo, action, &params[0], &params[1])
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
            }
            .into();

            mapinfo.insert(String::from(forwardtopic), newvalue.to_vec());
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
