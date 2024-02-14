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
use crate::engine::Engine;

pub type ConnectionReducer =
    Box<dyn Fn(ConnectionState, ActionMessage) -> ConnectionState + Send + 'static>;

pub struct ConnectionEngine(ConnectionReducer);

impl ConnectionEngine {
    pub fn new(reduce: ConnectionReducer) -> Self {
        Self(reduce)
    }
}

impl Engine<ActionMessage, ConnectionResult, ConnectionState> for ConnectionEngine {
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
// pub type ConnectionEngine = Engine<ActionMessage, ConnectionResult, ConnectionState>;

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

// pub fn create_engine(reduce: ConnectionReducer) -> ConnectionEngine {
//     Engine {
//         reduce,
//         template: Box::new(|state: &ConnectionState| ConnectionResult {
//             messages: state.messages.to_owned(),
//             is_final: state.is_final,
//         }),
//         is_final: Box::new(|result: &ConnectionResult| result.is_final),
//     }
// }
