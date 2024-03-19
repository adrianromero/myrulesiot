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

use serde_json::json;
use serde_json::Value;

use crate::mqtt::SliceFunction;
use crate::mqtt::SliceResult;
use crate::mqtt::{EngineAction, EngineMessage};

pub fn forward_user_action() -> SliceFunction {
    Box::new(
        |params: &Value, _info: &Value, action: &EngineAction| -> SliceResult {
            let topic = params["topic"].as_str().unwrap();
            let forwardtopic = params["forwardtopic"].as_str().unwrap();
            if action.matches(topic) {
                return SliceResult::messages(vec![EngineMessage::new(
                    String::from(forwardtopic),
                    action.payload.clone(),
                )]);
            }
            SliceResult::empty()
        },
    )
}

pub fn forward_action() -> SliceFunction {
    Box::new(
        |params: &serde_json::Value, info: &Value, action: &EngineAction| -> SliceResult {
            let topic = params["topic"].as_str().unwrap();
            let forwardtopic = params["forwardtopic"].as_str().unwrap();
            if action.matches(topic) {
                let json_payload: Value =
                    serde_json::from_slice(&action.payload).unwrap_or(json!(null));
                if json_payload["action"] == json!("toggle") {
                    let status = info[forwardtopic].as_bool();
                    let newvalue: bool = match status {
                        None => true,
                        Some(st) => !st,
                    };
                    return SliceResult::new(
                        json!({
                            forwardtopic : newvalue
                        }),
                        vec![EngineMessage::new(
                            String::from(forwardtopic),
                            if newvalue { vec![1] } else { vec![0] },
                        )],
                    );
                }
            }
            SliceResult::empty()
        },
    )
}
