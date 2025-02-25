//    MyRulesIoT is a rules engine for MQTT
//    Copyright (C) 2021-2025 Adrián Romero Corchado.
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

use linkme::distributed_slice;
use serde_json::{json, Value};

use crate::master::{EngineAction, SliceFunction, SliceResult};

use super::SLICEFUNCTIONS;

#[distributed_slice(SLICEFUNCTIONS)]
fn slice_start_action() -> (String, SliceFunction) {
    (String::from("start_action"), start_action())
}

pub fn start_action() -> SliceFunction {
    Box::new(|info: &Value, action: &EngineAction| -> SliceResult {
        let topic = info["_topic"].as_str().unwrap();
        let command = info["_command"].as_str().unwrap();
        //TODO: Only topic activates start if command null
        SliceResult::state(json!({ "_start" : action.matches_action(topic, command.as_bytes())}))
    })
}

#[distributed_slice(SLICEFUNCTIONS)]
fn slice_start_json_action() -> (String, SliceFunction) {
    (String::from("start_json_action"), start_json_action())
}

pub fn start_json_action() -> SliceFunction {
    Box::new(|info: &Value, action: &EngineAction| -> SliceResult {
        let topic = info["_topic"].as_str().unwrap();
        let pointer = info["_pointer"].as_str().unwrap();
        let value = &info["_value"];
        imp_start_json_action(info, action, topic, pointer, value)
    })
}

pub fn imp_start_json_action(
    _info: &Value,
    action: &EngineAction,
    topic: &str,
    pointer: &str,
    value: &Value,
) -> SliceResult {
    SliceResult::state(json!({ "_start" : action.matches(topic) && {
                let json_payload = serde_json::from_slice(&action.payload).unwrap_or(json!(null));
                json_payload.pointer(pointer).map_or(false, |v| v.eq(value))
            }
    }))
}
