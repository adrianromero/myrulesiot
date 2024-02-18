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

use crate::runtime::Engine;
use rumqttc::{Publish, QoS};

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

#[derive(Debug)]
pub struct ConnectionAction {
    pub topic: String,
    pub payload: Vec<u8>,
    pub timestamp: i64,
}

impl ConnectionAction {
    pub fn matches(&self, filter: &str) -> bool {
        rumqttc::matches(&self.topic, filter)
    }
    pub fn matches_action(&self, filter: &str, payload: Vec<u8>) -> bool {
        rumqttc::matches(&self.topic, filter) && payload.eq(&self.payload)
    }
}

impl From<Publish> for ConnectionAction {
    fn from(p: Publish) -> ConnectionAction {
        ConnectionAction {
            topic: p.topic,
            payload: p.payload.into(),
            timestamp: chrono::Local::now().timestamp_millis(),
        }
    }
}

#[derive(Debug)]
pub struct ConnectionResult {
    pub messages: Vec<ConnectionMessage>,
    pub is_final: bool,
}

pub struct ConnectionEngine<T: Fn(ConnectionState, ConnectionAction) -> ConnectionState>(T);

impl<T: Fn(ConnectionState, ConnectionAction) -> ConnectionState> ConnectionEngine<T> {
    pub fn new(reduce: T) -> Self {
        Self(reduce)
    }
}

impl<T: Fn(ConnectionState, ConnectionAction) -> ConnectionState>
    Engine<ConnectionAction, ConnectionResult, ConnectionState> for ConnectionEngine<T>
{
    fn reduce(&self, state: ConnectionState, action: ConnectionAction) -> ConnectionState {
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

#[derive(Debug, Clone)]
pub struct ConnectionMessage {
    pub qos: QoS,
    pub retain: bool,
    pub topic: String,
    pub payload: Vec<u8>,
}

pub type FnMQTTReducer =
    Box<dyn Fn(&mut HashMap<String, Vec<u8>>, &ConnectionAction) -> Vec<ConnectionMessage> + Send>;

pub fn create_reducer(
    reducers: Vec<FnMQTTReducer>,
) -> impl Fn(ConnectionState, ConnectionAction) -> ConnectionState {
    move |state: ConnectionState, action: ConnectionAction| {
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
