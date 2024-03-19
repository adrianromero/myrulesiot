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

use serde_json::{json, Value};

use crate::mqtt::{EngineAction, EngineMessage, SliceFunction, SliceResult};

pub fn relay() -> SliceFunction {
    Box::new(|params: &Value, mapinfo: &Value, _action: &EngineAction| {
        let topic = params["topic"].as_str().unwrap();
        let value = params["value"].as_str().unwrap().as_bytes();
        imp_relay(mapinfo, topic, value)
    })
}

pub fn relay_value(value: &[u8]) -> SliceFunction {
    let value: Vec<u8> = value.into();
    Box::new(
        move |params: &Value, mapinfo: &Value, _action: &EngineAction| {
            let topic = params["topic"].as_str().unwrap();
            imp_relay(mapinfo, topic, &value)
        },
    )
}

fn imp_relay(mapinfo: &serde_json::Value, topic: &str, value: &[u8]) -> SliceResult {
    if mapinfo["_actuator"] == json!(true) {
        return SliceResult::messages(vec![EngineMessage::new(String::from(topic), value.into())]);
    }
    SliceResult::empty()
}
