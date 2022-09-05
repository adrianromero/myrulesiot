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

use rumqttc::QoS;
use serde_json::json;
use serde_json::Value;

use crate::mqtt::{ActionMessage, ConnectionMessage};

pub fn actuator(actuatortopic: &str, action: &str) -> impl Fn(&ActionMessage) -> bool {
    let str_actuatortopic: String = actuatortopic.to_owned();
    let str_action: String = action.to_owned();

    move |action: &ActionMessage| -> bool {
        if action.matches(&str_actuatortopic) {
            let json_payload: Value =
                serde_json::from_slice(&action.payload).unwrap_or(json!(null));
            json_payload["action"] == json!(&str_action)
        } else {
            false
        }
    }
}

pub fn actuator_toggle(actuatortopic: &str) -> impl Fn(&ActionMessage) -> bool {
    actuator(actuatortopic, "toggle")
}

pub fn actuator_brightness_up(actuatortopic: &str) -> impl Fn(&ActionMessage) -> bool {
    actuator(actuatortopic, "brightness_up_click")
}

pub fn actuator_brightness_down(actuatortopic: &str) -> impl Fn(&ActionMessage) -> bool {
    actuator(actuatortopic, "brightness_down_click")
}

pub fn actuator_arrow_right(actuatortopic: &str) -> impl Fn(&ActionMessage) -> bool {
    actuator(actuatortopic, "arrow_right_click")
}

pub fn actuator_arrow_left(actuatortopic: &str) -> impl Fn(&ActionMessage) -> bool {
    actuator(actuatortopic, "arrow_left_click")
}

pub fn light_toggle(
    actionmatch: impl Fn(&ActionMessage) -> bool,
    strtopic: impl Into<String>,
) -> impl Fn(&mut HashMap<String, Vec<u8>>, &ActionMessage) -> Vec<ConnectionMessage> {
    let topic = strtopic.into();

    move |_mapinfo: &mut HashMap<String, Vec<u8>>,
          action: &ActionMessage|
          -> Vec<ConnectionMessage> {
        if actionmatch(action) {
            return vec![ConnectionMessage {
                topic: format!("{}/set", &topic),
                payload: "{\"state\":\"TOGGLE\"}".into(),
                qos: QoS::AtMostOnce,
                retain: false,
            }];
        }

        vec![]
    }
}
