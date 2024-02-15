//    MyRulesIoT is a rules engine for MQTT
//    Copyright (C) 2021 Adrián Romero Corchado.
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

use super::{ActionMessage, ConnectionMessage, ConnectionResult};
use crate::runtime::Engine;

pub struct ConnectionEngine<T: Fn(ConnectionState, ActionMessage) -> ConnectionState>(T);

impl<T: Fn(ConnectionState, ActionMessage) -> ConnectionState> ConnectionEngine<T> {
    pub fn new(reduce: T) -> Self {
        Self(reduce)
    }
}

impl<T: Fn(ConnectionState, ActionMessage) -> ConnectionState>
    Engine<ActionMessage, ConnectionResult, ConnectionState> for ConnectionEngine<T>
{
    fn reduce(&self, state: ConnectionState, action: ActionMessage) -> ConnectionState {
        self.0(state, action)
    }
    fn template(&self, state: &ConnectionState) -> ConnectionResult {
        ConnectionResult {
            messages: state.messages.to_owned(),
            is_final: state.is_final,
        }
    }
    fn is_final(&self, result: &ConnectionResult) -> bool {
        result.is_final
    }
}

#[derive(Debug)]
pub struct ConnectionState {
    pub info: HashMap<String, Vec<u8>>,
    pub messages: Vec<ConnectionMessage>,
    pub is_final: bool,
}

impl Default for ConnectionState {
    fn default() -> Self {
        ConnectionState {
            info: Default::default(),
            messages: vec![],
            is_final: false,
        }
    }
}

pub type FnMQTTReducer =
    Box<dyn Fn(&mut HashMap<String, Vec<u8>>, &ActionMessage) -> Vec<ConnectionMessage> + Send>;

pub fn create_reducer(
    reducers: Vec<FnMQTTReducer>,
) -> impl Fn(ConnectionState, ActionMessage) -> ConnectionState {
    move |state: ConnectionState, action: ActionMessage| {
        let mut messages = Vec::<ConnectionMessage>::new();
        let mut newmap = state.info.clone();

        for f in &reducers {
            messages.append(&mut f(&mut newmap, &action));
        }

        let is_final = action.matches_action("SYSMR/system_action", "exit".into());

        ConnectionState {
            info: newmap,
            messages,
            is_final,
        }
    }
}
