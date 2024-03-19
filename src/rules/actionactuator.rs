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

use crate::mqtt::{EngineAction, SliceFunction, SliceResult};

pub fn actuator_action() -> SliceFunction {
    Box::new(
        |params: &Value, _info: &Value, action: &EngineAction| -> SliceResult {
            let topic = params["topic"].as_str().unwrap();
            let command = params["command"].as_str().unwrap();
            //TODO: Only topic activates actuator if command null
            SliceResult::state(
                json!({ "_actuator" : action.matches_action(topic, command.as_bytes())}),
            )
        },
    )
}

pub fn actuator_json_action() -> SliceFunction {
    Box::new(
        |params: &Value, info: &Value, action: &EngineAction| -> SliceResult {
            let topic = params["topic"].as_str().unwrap();
            let pointer = params["pointer"].as_str().unwrap();
            let value = &params["value"];
            imp_actuator_json_action(info, action, topic, pointer, value)
        },
    )
}

pub fn imp_actuator_json_action(
    _info: &Value,
    action: &EngineAction,
    topic: &str,
    pointer: &str,
    value: &Value,
) -> SliceResult {
    SliceResult::state(json!({ "_actuator" : action.matches(topic) && {
                let json_payload = serde_json::from_slice(&action.payload).unwrap_or(json!(null));
                json_payload.pointer(pointer).map_or(false, |v| v.eq(value))
            }
    }))
}
