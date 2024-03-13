//    MyRulesIoT is a rules engine for MQTT
//    Copyright (C) 2024 Adrián Romero Corchado.
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

use super::{EngineAction, EngineMessage, EngineResult};
use crate::runtime::Engine;

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReducerFunction {
    name: String,
    parameters: serde_json::Value,
}

impl ReducerFunction {
    pub fn new(name: String, parameters: serde_json::Value) -> Self {
        ReducerFunction { name, parameters }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EngineState {
    pub info: serde_json::Value,
    pub functions: Vec<ReducerFunction>,
    pub messages: Vec<EngineMessage>,
    pub is_final: bool,
}

impl Default for EngineState {
    fn default() -> Self {
        EngineState {
            info: json!({}),
            functions: vec![],
            messages: vec![],
            is_final: false,
        }
    }
}

impl EngineState {
    pub fn new(info: serde_json::Value, functions: Vec<ReducerFunction>) -> Self {
        EngineState {
            info,
            functions,
            messages: vec![],
            is_final: false,
        }
    }
}

pub type EngineFunction = fn(
    loopstack: &mut serde_json::Value,
    mapinfo: &mut serde_json::Value,
    action: &EngineAction,
    parameters: &serde_json::Value,
) -> Vec<EngineMessage>;

pub struct MasterEngine {
    prefix_id: String,
    engine_functions: HashMap<String, EngineFunction>,
}

impl MasterEngine {
    pub fn new(prefix_id: String, engine_functions: HashMap<String, EngineFunction>) -> Self {
        Self {
            prefix_id,
            engine_functions,
        }
    }
}

impl Engine<EngineAction, EngineResult, EngineState> for MasterEngine {
    fn reduce(&self, state: EngineState, action: EngineAction) -> EngineState {
        let mut messages = Vec::<EngineMessage>::new();
        let mut info = state.info.clone();
        let mut functions = state.functions.clone();
        let mut is_final = false;

        if action.matches(&format!("{}/command/functions_push", self.prefix_id)) {
            match serde_json::from_slice::<ReducerFunction>(&action.payload) {
                Ok(f) => {
                    let function_name = f.name.clone();
                    functions.push(f);
                    messages.push(EngineMessage::new_json(
                        format!("{}/notify/functions_push", self.prefix_id),
                        json!({
                          "success" : true,
                          "function" : format!("{}", function_name)
                        }),
                    ));
                }
                Err(error) => messages.push(EngineMessage::new_json(
                    format!("{}/notify/system_error", self.prefix_id),
                    json!({
                      "command" : "functions_push",
                      "error" : format!("{}", error.to_string())
                    }),
                )),
            }
        } else if action.matches(&format!("{}/command/functions_pop", self.prefix_id)) {
            functions.pop();
            messages.push(EngineMessage::new_json(
                format!("{}/notify/functions_pop", self.prefix_id),
                json!({
                  "success" : true,
                }),
            ));
        } else if action.matches(&format!("{}/command/functions_clear", self.prefix_id)) {
            functions.clear();
            messages.push(EngineMessage::new_json(
                format!("{}/notify/functions_clear", self.prefix_id),
                json!({
                  "success" : true,
                }),
            ));
        } else if action.matches(&format!("{}/command/functions_putall", self.prefix_id)) {
            match serde_json::from_slice(&action.payload) {
                Ok(fns) => {
                    functions = fns;
                    messages.push(EngineMessage::new_json(
                        format!("{}/notify/functions_putall", self.prefix_id),
                        json!({
                          "success" : true,
                        }),
                    ));
                }
                Err(error) => messages.push(EngineMessage::new_json(
                    format!("{}/notify/system_error", self.prefix_id),
                    json!({
                      "command" : "functions_putall",
                      "error" : format!("{}", error.to_string())
                    }),
                )),
            }
        } else if action.matches(&format!("{}/command/functions_getall", self.prefix_id)) {
            messages.push(EngineMessage::new_json(
                format!("{}/notify/functions_getall", self.prefix_id),
                serde_json::to_value(&functions).unwrap(),
            ));
        } else if action.matches(&format!("{}/command/send_messages", self.prefix_id)) {
            messages = state.messages;
        } else if action.matches(&format!("{}/command/exit", self.prefix_id)) {
            is_final = true;
        } else {
            let mut loopstack = json!({});
            log::debug!("executing {} functions)", functions.len());
            for fun in &functions {
                log::debug!("executing {}({})", fun.name, fun.parameters);
                let func = self.engine_functions.get(&fun.name);
                match func {
                    Some(f) => {
                        messages.append(&mut f(&mut loopstack, &mut info, &action, &fun.parameters))
                    }
                    None => messages.push(EngineMessage::new(
                        format!("{}/notify/system_error", self.prefix_id),
                        format!("Function not found: {}", &fun.name).into(),
                    )),
                }
            }
        }

        if is_final {
            messages.push(EngineMessage::new_json(
                format!("{}/notify/exit_functions", self.prefix_id),
                serde_json::to_value(&functions).unwrap(),
            ));
            messages.push(EngineMessage::new_json(
                format!("{}/notify/exit", self.prefix_id),
                json!({
                  "success" : true,
                }),
            ));
        }

        EngineState {
            info,
            functions,
            messages,
            is_final,
        }
    }
    fn template(&self, state: &EngineState) -> EngineResult {
        EngineResult {
            messages: state.messages.clone(),
            is_final: state.is_final,
        }
    }
    fn is_final(&self, result: &EngineResult) -> bool {
        result.is_final
    }
}
