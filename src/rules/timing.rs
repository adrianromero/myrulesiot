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

use crate::mqtt::{EngineAction, SliceFunction, SliceResult};

pub fn condition_sleep() -> SliceFunction {
    Box::new(
        |params: &Value, info: &Value, action: &EngineAction| -> SliceResult {
            let millis = params["millis"].as_i64().unwrap_or(1000);
            let timeindex = &format!("condition_sleep_{}", info["_index"]);

            if info["_actuator"] == json!(true) {
                return SliceResult::state(json!({
                    timeindex: action.timestamp,
                    "_actuator" : null
                }));
            }

            if let Some(activation) = &info[timeindex].as_i64() {
                if action.timestamp - activation > millis {
                    return SliceResult::state(json!({
                        timeindex : null,
                        "_actuator" : true
                    }));
                }
            }

            return SliceResult::state(json!({
                "_actuator": null
            }));
        },
    )
}
