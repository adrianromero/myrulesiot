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

use crate::master::{EngineAction, EngineMessage, SliceResult};

pub fn save_list(info: &Value, action: &EngineAction) -> SliceResult {
    let topic = info["_topic"].as_str().unwrap();
    let duration = info["_value"].as_i64().unwrap();
    let count = info["_count"].as_i64().unwrap() as usize;

    let time_tick: i64 = duration / count as i64;
    let timestamp = info["_timestamp"].as_i64().unwrap();

    let topic_store = format!("{}/list", topic);

    if action.matches(&topic) {
        match serde_json::from_slice::<Value>(&action.payload) {
            Ok(value) => {
                return SliceResult::state(json!({
                    &topic_store: {
                        "current" : value,
                    }
                }));
            }
            Err(_) => {
                return SliceResult::state(json!({
                    &topic_store: {
                        "current" : "error",
                    }
                }));
            }
        }
    }

    let list = &info[&topic_store];
    let current: &Value = &list["current"];
    let valuest: &Value = &list["valuest"];

    if action.matches("SYSMR/action/tick") {
        match valuest.as_i64() {
            None => {
                let mut values: Vec<Value> = vec![Value::Null; count];
                let valuest = timestamp;
                if let Some(last) = values.last_mut() {
                    *last = current.clone();
                }
                return SliceResult::new(
                    json!({
                        &topic_store: {
                            "valuest" : valuest,
                            "values" : values,
                        }
                    }),
                    vec![EngineMessage::new(
                        topic_store,
                        json!(values).to_string().into(),
                    )],
                );
            }
            Some(t) => {
                let mut values: Vec<Value> = list["values"].as_array().unwrap().clone();
                let mut valuest = t;
                if timestamp >= t + time_tick {
                    while timestamp >= valuest + time_tick {
                        valuest += time_tick;
                        values.rotate_left(1);
                        if let Some(last) = values.last_mut() {
                            *last = current.clone();
                        }
                    }
                    return SliceResult::new(
                        json!({
                            &topic_store: {
                                "valuest" : valuest,
                                "values" : values,
                            }
                        }),
                        vec![EngineMessage::new(
                            topic_store,
                            json!(values).to_string().into(),
                        )],
                    );
                }
            }
        }
    }
    SliceResult::empty()
}

pub fn save_value(info: &Value, action: &EngineAction) -> SliceResult {
    let topic = info["_topic"].as_str().unwrap();
    if action.matches(topic) {
        return SliceResult::state(json!({
            &format!("{}/store", topic) : action.payload
        }));
    }
    SliceResult::empty()
}
