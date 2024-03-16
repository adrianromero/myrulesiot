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

use serde_json::{json, Value};

use crate::mqtt::{EngineAction, EngineMessage};

pub fn actuator_action(
    loopstack: &mut serde_json::Value,
    _mapinfo: &mut serde_json::Value,
    action: &EngineAction,
    params: &serde_json::Value,
) -> Vec<EngineMessage> {
    let topic = params["topic"].as_str().unwrap();
    let command = params["command"].as_str().unwrap();
    loopstack["actuator"] = json!(action.matches_action(topic, command.as_bytes()));
    vec![]
}

pub fn imp_actuator_json_action(
    loopstack: &mut serde_json::Value,
    action: &EngineAction,
    topic: &str,
    pointer: &str,
    value: &Value,
) -> Vec<EngineMessage> {
    loopstack["actuator"] = json!(
        action.matches(topic) && {
            let json_payload = serde_json::from_slice(&action.payload).unwrap_or(json!(null));
            json_payload.pointer(pointer).map_or(false, |v| v.eq(value))
        }
    );

    vec![]
}

pub fn actuator_json_action(
    loopstack: &mut serde_json::Value,
    _mapinfo: &mut serde_json::Value,
    action: &EngineAction,
    params: &serde_json::Value,
) -> Vec<EngineMessage> {
    let topic = params["topic"].as_str().unwrap();
    let pointer = params["pointer"].as_str().unwrap();
    let value = &params["value"];
    imp_actuator_json_action(loopstack, action, topic, pointer, value)
}

// command values
// "toggle"
// "brightness_up_click"
// "brightness_down_click"
// "arrow_right_click"
// "arrow_left_click"

pub fn actuator_ikea_remote_toggle(
    loopstack: &mut serde_json::Value,
    _mapinfo: &mut serde_json::Value,
    action: &EngineAction,
    params: &serde_json::Value,
) -> Vec<EngineMessage> {
    let topic = params["topic"].as_str().unwrap();
    imp_actuator_json_action(loopstack, action, topic, "/action", &json!("toggle"))
}
