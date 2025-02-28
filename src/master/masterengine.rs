//    MyRulesIoT is a rules engine for MQTT
//    Copyright (C) 2024-2025 Adrián Romero Corchado.
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

use crate::runtime::Engine;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReducerFunction {
    name: String,
    #[serde(flatten)]
    parameters: Value,
}

impl ReducerFunction {
    pub fn new(name: String, parameters: Value) -> Self {
        ReducerFunction { name, parameters }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EngineState {
    pub info: Value,
    pub functions: Vec<ReducerFunction>,
}

impl Default for EngineState {
    fn default() -> Self {
        EngineState {
            info: json!({}),
            functions: vec![],
        }
    }
}

impl EngineState {
    pub fn new(info: Value, functions: Vec<ReducerFunction>) -> Self {
        EngineState { info, functions }
    }
}

#[derive(Debug)]
pub struct EngineAction {
    pub topic: String,
    pub payload: Vec<u8>,
}

impl EngineAction {
    pub fn new(topic: String, payload: Vec<u8>) -> Self {
        EngineAction { topic, payload }
    }
    pub fn new_json(topic: String, payload: Value) -> Self {
        EngineAction {
            topic,
            payload: payload.to_string().into_bytes(),
        }
    }
    pub fn matches(&self, filter: &str) -> bool {
        self.topic.eq(filter)
    }
    pub fn matches_action(&self, filter: &str, payload: &[u8]) -> bool {
        self.topic.eq(filter) && payload.eq(&self.payload)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineMessage {
    pub topic: String,
    pub payload: Vec<u8>,
    pub properties: Value,
}

impl EngineMessage {
    pub fn new(topic: String, payload: Vec<u8>) -> Self {
        EngineMessage {
            topic,
            payload,
            properties: Value::Null,
        }
    }

    pub fn new_json<T>(topic: String, payload: &T) -> Self
    where
        T: ?Sized + Serialize,
    {
        EngineMessage {
            topic,
            payload: serde_json::to_vec(payload).unwrap(),
            properties: Value::Null,
        }
    }

    pub fn new_jsonpretty<T>(topic: String, payload: &T) -> Self
    where
        T: ?Sized + Serialize,
    {
        EngineMessage {
            topic,
            payload: serde_json::to_vec_pretty(payload).unwrap(),
            properties: Value::Null,
        }
    }

    pub fn payload_into_json(&self) -> serde_json::Result<Value> {
        serde_json::from_slice::<Value>(&self.payload)
    }
}

#[derive(Debug, Clone)]
pub struct EngineResult {
    pub messages: Vec<EngineMessage>,
    pub is_final: bool,
}

pub struct SliceResult {
    pub state: Value,
    pub messages: Vec<EngineMessage>,
}

impl SliceResult {
    pub fn empty() -> Self {
        SliceResult {
            state: json!({}),
            messages: vec![],
        }
    }
    pub fn messages(messages: Vec<EngineMessage>) -> Self {
        SliceResult {
            state: json!({}),
            messages,
        }
    }
    pub fn state(state: Value) -> Self {
        SliceResult {
            state,
            messages: vec![],
        }
    }
    pub fn new(state: Value, messages: Vec<EngineMessage>) -> Self {
        SliceResult { state, messages }
    }
}

pub type SliceFunction = Box<dyn Fn(&Value, &EngineAction) -> SliceResult + Send>;

#[derive(Serialize, Deserialize, Debug)]
pub enum FinalStatus {
    NORMAL,
    ERROR,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EngineStatus {
    RUNNING,
    FINAL(FinalStatus, String),
}

pub struct MasterEngine {
    prefix_id: String,
    engine_functions: HashMap<String, SliceFunction>,
}

impl MasterEngine {
    pub fn new(prefix_id: String, engine_functions: HashMap<String, SliceFunction>) -> Self {
        Self {
            prefix_id,
            engine_functions,
        }
    }
}

impl Engine<EngineAction, EngineResult, EngineState> for MasterEngine {
    fn reduce(&self, state: EngineState, action: EngineAction) -> (EngineState, EngineResult) {
        let mut messages = Vec::<EngineMessage>::new();
        let mut info = state.info;
        let mut functions = state.functions;
        let mut engine_status: EngineStatus = EngineStatus::RUNNING;

        if action.matches(&format!("{}/command/functions_push", self.prefix_id)) {
            match serde_json::from_slice::<ReducerFunction>(&action.payload) {
                Ok(f) => {
                    messages.push(EngineMessage::new_json(
                        format!("{}/notify/functions_push", self.prefix_id),
                        &json!({
                          "success" : true,
                          "function" : f.name
                        }),
                    ));
                    functions.push(f);
                }
                Err(error) => {
                    log::warn!("functions_push: Not a ReducerFunction.");
                    messages.push(EngineMessage::new_json(
                        format!("{}/notify/system_error", self.prefix_id),
                        &json!({
                          "command" : "functions_push",
                          "error" : format!("{}", error.to_string())
                        }),
                    ))
                }
            }
        } else if action.matches(&format!("{}/command/functions_pop", self.prefix_id)) {
            let f = functions.pop();
            messages.push(EngineMessage::new_json(
                format!("{}/notify/functions_pop", self.prefix_id),
                &json!({
                  "success" : true,
                  "function" : f.map_or_else(|| String::from("<None>"), |f| f.name)
                }),
            ));
        } else if action.matches(&format!("{}/command/functions_clear", self.prefix_id)) {
            functions.clear();
            messages.push(EngineMessage::new_json(
                format!("{}/notify/functions_clear", self.prefix_id),
                &json!({
                  "success" : true,
                }),
            ));
        } else if action.matches(&format!("{}/command/functions_putall", self.prefix_id)) {
            match serde_json::from_slice(&action.payload) {
                Ok(fns) => {
                    functions = fns;
                    messages.push(EngineMessage::new_json(
                        format!("{}/notify/functions_putall", self.prefix_id),
                        &json!({
                          "success" : true,
                        }),
                    ));
                }
                Err(error) => {
                    log::warn!("functions_putall: Not a list of ReducerFunction.");
                    messages.push(EngineMessage::new_json(
                        format!("{}/notify/system_error", self.prefix_id),
                        &json!({
                          "command" : "functions_putall",
                          "error" : format!("{}", error.to_string())
                        }),
                    ))
                }
            }
        } else if action.matches(&format!("{}/command/functions_getall", self.prefix_id)) {
            messages.push(EngineMessage::new_json(
                format!("{}/notify/functions_getall", self.prefix_id),
                &functions,
            ));
        } else if action.matches(&format!("{}/command/exit", self.prefix_id)) {
            engine_status = EngineStatus::FINAL(
                FinalStatus::NORMAL,
                String::from_utf8(action.payload).unwrap_or_else(|utferror| utferror.to_string()),
            );
        } else if action.matches("SYSMR/action/error") {
            let final_message =
                String::from_utf8(action.payload).unwrap_or_else(|utferror| utferror.to_string());
            log::error!(
                "System Master Engine error. Received error {:?}",
                final_message
            );
            engine_status = EngineStatus::FINAL(FinalStatus::ERROR, final_message);
        } else if action.matches("SYSMR/action/load_functions") {
            match serde_json::from_slice(&action.payload) {
                Ok(fns) => {
                    functions = fns;
                }
                Err(error) => {
                    log::error!("System Master Engine error. Not a list of ReducerFunction in load_functions. {}", error);
                }
            }
        } else {
            log::debug!("executing {} functions)", functions.len());
            if let Value::Object(obj) = &mut info {
                let utc = chrono::Utc::now().timestamp_millis();
                obj.insert("_timestamp".into(), json!(utc));
            }
            for (i, fun) in functions.iter().enumerate() {
                log::debug!("executing {}-{}({})", i, fun.name, fun.parameters);

                if let Value::Object(obj) = &mut info {
                    obj.insert("_index".into(), json!(i));
                }

                let func = self.engine_functions.get(&fun.name);
                match func {
                    Some(f) => {
                        json_patch::merge(&mut info, &fun.parameters);
                        let mut result = f(&mut info, &action);
                        json_patch::merge(&mut info, &result.state);
                        messages.append(&mut result.messages);
                    }
                    None => {
                        log::warn!("Function not found: {}", fun.name);
                        messages.push(EngineMessage::new(
                            format!("{}/notify/system_error", self.prefix_id),
                            format!("Function not found: {}", &fun.name).into(),
                        ))
                    }
                }
            }
            // Removes all non persitable keys
            if let Value::Object(obj) = &mut info {
                obj.retain(|k: &String, _v: &mut Value| !k.starts_with("_"));
            }
        }

        let is_final = match engine_status {
            EngineStatus::RUNNING => false,
            EngineStatus::FINAL(final_status, message) => {
                messages.push(EngineMessage::new_jsonpretty(
                    "SYSMR/notify/save_functions".into(),
                    &functions,
                ));
                let f = match final_status {
                    FinalStatus::NORMAL => "NORMAL",
                    FinalStatus::ERROR => "ERROR",
                };
                messages.push(EngineMessage::new(
                    format!("{}/notify/exit", self.prefix_id),
                    format!("{}/{}", f, message).into_bytes(),
                ));
                true
            }
        };

        (
            EngineState { info, functions },
            EngineResult { messages, is_final },
        )
    }
    fn is_final(&self, result: &EngineResult) -> bool {
        result.is_final
    }
}
