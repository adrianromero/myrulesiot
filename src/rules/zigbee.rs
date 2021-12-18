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

use crate::mqtt::ActionMessage;
use serde_json::json;
use serde_json::Value;

pub fn actuator(actuatortopic: &str, action: &str) -> impl FnOnce(&ActionMessage) -> bool {
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

pub fn actuator_toggle(actuatortopic: &str) -> impl FnOnce(&ActionMessage) -> bool {
    actuator(actuatortopic, "toggle")
}

// pub fn forward_action(
//     stractiontopic: &str,
//     strtopic: &str,
// ) -> impl FnOnce(&mut HashMap<String, Vec<u8>>, &ActionMessage) -> Vec<ConnectionMessage> {
//     let topic_value: String = strtopic.to_owned();
//     let action_topic_value: String = stractiontopic.to_owned();

//     move |mapinfo: &mut HashMap<String, Vec<u8>>,
//           action: &ActionMessage|
//           -> Vec<ConnectionMessage> {
//         if action.matches(&action_topic_value) {
//             let json_payload: Value =
//                 serde_json::from_slice(&action.payload).unwrap_or(json!(null));
//             if json_payload["action"] == json!("toggle") {
//                 let status = mapinfo
//                     .get(&topic_value)
//                     .map(|s| String::from_utf8_lossy(&s));

//                 let newvalue: Bytes = match status {
//                     None => "1",
//                     Some(st) => {
//                         if st == "1" {
//                             "0"
//                         } else {
//                             "1"
//                         }
//                     }
//                 }
//                 .into();

//                 mapinfo.insert(topic_value.clone(), newvalue.to_vec());
//                 return vec![ConnectionMessage {
//                     topic: topic_value,
//                     payload: newvalue,
//                     qos: QoS::AtMostOnce,
//                     retain: false,
//                 }];
//             }
//         }
//         return vec![];
//     }
// }
