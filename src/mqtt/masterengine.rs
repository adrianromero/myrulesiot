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
use std::collections::HashMap;

#[derive(Debug)]
pub struct ReducerFunction {
    name: String,
    parameters: Vec<String>,
}

impl ReducerFunction {
    pub fn new(name: String, parameters: Vec<String>) -> Self {
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
    params: &[String],
) -> Vec<EngineMessage>;

pub struct MasterEngine<T: Fn(EngineState, EngineAction) -> EngineState>(T);

impl<T: Fn(EngineState, EngineAction) -> EngineState> MasterEngine<T> {
    pub fn new(reduce: T) -> Self {
        Self(reduce)
    }
}

impl<T: Fn(EngineState, EngineAction) -> EngineState>
    Engine<EngineAction, EngineResult, EngineState> for MasterEngine<T>
{
    fn reduce(&self, state: EngineState, action: EngineAction) -> EngineState {
        self.0(state, action)
    }
    fn template(&self, state: &EngineState) -> EngineResult {
        EngineResult {
            messages: state.messages.to_owned(),
            is_final: state.is_final,
        }
    }
    fn is_final(&self, result: &EngineResult) -> bool {
        result.is_final
    }
}

pub fn create_engine_reducer(
    engine_functions: HashMap<String, EngineFunction>,
) -> impl Fn(EngineState, EngineAction) -> EngineState {
    move |state: EngineState, action: EngineAction| {
        let mut messages = Vec::<EngineMessage>::new();
        let mut newmap = state.info.clone();
        let functions = state.reducers;

        for fun in &functions {
            let func = engine_functions.get(&fun.name);
            match func {
                Some(f) => messages.append(&mut f(&mut newmap, &action, &fun.parameters)),
                None => messages.push(EngineMessage {
                    topic: String::from("SYSMR/system_error"),
                    payload: format!("Function not found: {}", &fun.name).into(),
                    qos: QoS::AtMostOnce,
                    retain: false,
                }),
            }
        }

        let is_final = action.matches_action("SYSMR/system_action", "exit".into());

        EngineState {
            info: newmap,
            reducers: functions,
            messages,
            is_final,
        }
    }
}
