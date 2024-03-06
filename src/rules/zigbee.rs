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

use crate::mqtt::{from_qos, EngineAction, EngineMessage};

pub fn engine_ikea_actuator(
    loopstack: &mut serde_json::Value,
    mapinfo: &mut serde_json::Value,
    action: &EngineAction,
    params: &serde_json::Value,
) -> Vec<EngineMessage> {
    let topic = params["topic"].as_str().unwrap();
    let command = params["command"].as_str().unwrap();
    ikea_actuator(loopstack, mapinfo, action, topic, command)
}

// command values
// "toggle"
// "brightness_up_click"
// "brightness_down_click"
// "arrow_right_click"
// "arrow_left_click"

fn ikea_actuator(
    loopstack: &mut serde_json::Value,
    _mapinfo: &mut serde_json::Value,
    action: &EngineAction,
    topic: &str,
    command: &str,
) -> Vec<EngineMessage> {
    if action.matches(topic) {
        let json_payload: Value = serde_json::from_slice(&action.payload).unwrap_or(json!(null));
        let actuator = json_payload["action"] == json!(command);
        loopstack["actuator"] = json!(actuator);

        log::info!("actuator payload {}", json_payload);
    }
    vec![]
}

pub fn engine_shelly_relay(
    loopstack: &mut serde_json::Value,
    mapinfo: &mut serde_json::Value,
    action: &EngineAction,
    params: &serde_json::Value,
) -> Vec<EngineMessage> {
    let topic = params["topic"].as_str().unwrap();
    shelly_relay(loopstack, mapinfo, action, topic)
}

fn shelly_relay(
    loopstack: &mut serde_json::Value,
    _mapinfo: &mut serde_json::Value,
    _action: &EngineAction,
    topic: &str,
) -> Vec<EngineMessage> {
    if loopstack["actuator"] == json!(true) {
        return vec![EngineMessage {
            topic: String::from(topic),
            payload: b"on".into(),
            qos: from_qos(QoS::AtMostOnce),
            retain: false,
        }];
    }
    vec![]
}
