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

use super::{EngineAction, EngineMessage, EngineResult};
use crate::runtime::Engine;

#[derive(Debug)]
pub struct ConnectionState {
    pub info: HashMap<String, Vec<u8>>,
    pub functions: Vec<(String, Vec<String>)>,
    pub messages: Vec<EngineMessage>,
    pub is_final: bool,
}

impl Default for ConnectionState {
    fn default() -> Self {
        ConnectionState {
            info: Default::default(),
            functions: vec![],
            messages: vec![],
            is_final: false,
        }
    }
}

pub struct ConnectionEngine<T: Fn(ConnectionState, EngineAction) -> ConnectionState>(T);

impl<T: Fn(ConnectionState, EngineAction) -> ConnectionState> ConnectionEngine<T> {
    pub fn new(reduce: T) -> Self {
        Self(reduce)
    }
}

impl<T: Fn(ConnectionState, EngineAction) -> ConnectionState>
    Engine<EngineAction, EngineResult, ConnectionState> for ConnectionEngine<T>
{
    fn reduce(&self, state: ConnectionState, action: EngineAction) -> ConnectionState {
        self.0(state, action)
    }
    fn template(&self, state: &ConnectionState) -> EngineResult {
        EngineResult {
            messages: state.messages.to_owned(),
            is_final: state.is_final,
        }
    }
    fn is_final(&self, result: &EngineResult) -> bool {
        result.is_final
    }
}

pub type FnMQTTReducer =
    Box<dyn Fn(&mut HashMap<String, Vec<u8>>, &EngineAction) -> Vec<EngineMessage> + Send>;

pub fn create_reducer(
    reducers: Vec<FnMQTTReducer>,
) -> impl Fn(ConnectionState, EngineAction) -> ConnectionState {
    move |state: ConnectionState, action: EngineAction| {
        let mut messages = Vec::<EngineMessage>::new();
        let mut newmap = state.info.clone();

        for f in &reducers {
            messages.append(&mut f(&mut newmap, &action));
        }

        let is_final = action.matches_action("SYSMR/system_action", "exit".into());

        ConnectionState {
            info: newmap,
            functions: vec![],
            messages,
            is_final,
        }
    }
}
