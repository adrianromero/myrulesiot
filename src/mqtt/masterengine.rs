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
use rumqttc::QoS;
use serde::{Deserialize, Serialize};
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

#[derive(Debug)]
pub struct EngineState {
    pub info: HashMap<String, Vec<u8>>,
    pub reducers: Vec<ReducerFunction>,
    pub messages: Vec<EngineMessage>,
    pub is_final: bool,
}

impl Default for EngineState {
    fn default() -> Self {
        EngineState {
            info: Default::default(),
            reducers: vec![],
            messages: vec![],
            is_final: false,
        }
    }
}

impl EngineState {
    pub fn new(info: HashMap<String, Vec<u8>>, reducers: Vec<ReducerFunction>) -> Self {
        EngineState {
            info,
            reducers,
            messages: vec![],
            is_final: false,
        }
    }
}

pub type EngineFunction = fn(
    mapinfo: &mut HashMap<String, Vec<u8>>,
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
        let mut newmap = state.info.clone();
        let mut functions = state.reducers.clone();
        let mut is_final = false;
        if action.matches(&format!("{}/command/functions_push", self.prefix_id)) {
            let f: ReducerFunction = serde_json::from_slice(&action.payload).unwrap();
            functions.push(f);
        } else if action.matches(&format!("{}/command/functions_pop", self.prefix_id)) {
            functions.pop();
        } else if action.matches(&format!("{}/command/functions_clear", self.prefix_id)) {
            functions.clear();
        } else if action.matches(&format!("{}/command/functions_list", self.prefix_id)) {
            messages.push(EngineMessage {
                topic: format!("{}/notify/system_error", self.prefix_id),
                payload: serde_json::to_string(&functions).unwrap().into_bytes(),
                qos: QoS::AtMostOnce,
                retain: false,
            });
        } else if action.matches(&format!("{}/command/exit", self.prefix_id)) {
            is_final = true;
        } else {
            for fun in &functions {
                let func = self.engine_functions.get(&fun.name);
                match func {
                    Some(f) => messages.append(&mut f(&mut newmap, &action, &fun.parameters)),
                    None => messages.push(EngineMessage {
                        topic: format!("{}/notify/system_error", self.prefix_id),
                        payload: format!("Function not found: {}", &fun.name).into(),
                        qos: QoS::AtMostOnce,
                        retain: false,
                    }),
                }
            }
        }
        EngineState {
            info: newmap,
            reducers: functions,
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
