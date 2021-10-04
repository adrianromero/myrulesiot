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

use super::{ConnectionMessage, ConnectionResult};
use crate::engine::Engine;

pub type ConnectionReducer<S> = fn(&ConnectionState<S>, &ConnectionMessage) -> ConnectionState<S>;

pub type ConnectionEngine<S> = Engine<ConnectionMessage, ConnectionResult, ConnectionState<S>>;

#[derive(Debug)]
pub struct ConnectionState<S> {
    pub info: S,
    pub messages: Vec<ConnectionMessage>,
    pub is_final: bool,
}

impl<S: Default> Default for ConnectionState<S> {
    fn default() -> Self {
        ConnectionState {
            info: Default::default(),
            messages: vec![],
            is_final: false,
        }
    }
}

pub fn create_engine<S>(reduce: ConnectionReducer<S>) -> ConnectionEngine<S> {
    Engine {
        reduce: reduce,
        template: |state: &ConnectionState<S>| ConnectionResult {
            messages: state.messages.to_owned(),
            is_final: state.is_final,
        },
        is_final: |result: &ConnectionResult| result.is_final,
    }
}
