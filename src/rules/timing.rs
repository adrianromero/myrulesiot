//    MyRulesIoT is a rules engine for MQTT
//    Copyright (C) 2021-2025 Adrián Romero Corchado.
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

use linkme::distributed_slice;
use serde_json::{json, Value};

use super::SLICEFUNCTIONS;
use crate::master::{EngineAction, SliceFunction, SliceResult};

#[distributed_slice(SLICEFUNCTIONS)]
fn _condition_sleep() -> (String, SliceFunction) {
    (String::from("condition_sleep"), condition_sleep())
}
pub fn condition_sleep() -> SliceFunction {
    Box::new(|info: &Value, _action: &EngineAction| -> SliceResult {
        let millis = info["_millis"].as_i64().unwrap_or(1000);
        let timeindex = &format!("condition_sleep_{}", info["_index"]);
        let timestamp = info["_timestamp"].as_i64().unwrap();

        if info["_start"] == json!(true) {
            return SliceResult::state(json!({
                timeindex: timestamp,
                "_start" : null
            }));
        }

        if let Some(activation) = &info[timeindex].as_i64() {
            if timestamp - activation > millis {
                return SliceResult::state(json!({
                    timeindex : null,
                    "_start" : true
                }));
            }
        }

        return SliceResult::state(json!({
            "_start": null
        }));
    })
}
