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

pub mod forward;
pub mod lights;
pub mod savelist;
pub mod zigbee;

// pub fn light_actions(
//     strtopic: &str,
// ) -> impl FnOnce(&mut HashMap<String, Vec<u8>>, &ActionMessage) -> Vec<EngineMessage> {
//     let topic = strtopic.to_string();
//     move |mapinfo: &mut HashMap<String, Vec<u8>>,
//           action: &ActionMessage|
//           -> Vec<EngineMessage> {
//         //LightStatus temporizator
//         let topic_temp = topic.clone() + "/temp";
//         if action.matches(&topic_temp) {
//             let smillis = String::from_utf8_lossy(&action.payload);
//             let millis: i64 = smillis.parse().unwrap_or(5000);
//             mapinfo.insert(
//                 topic.clone(),
//                 bincode::serialize(&LightStatus {
//                     temp: Some(action.timestamp + millis),
//                     value: "1".to_string(),
//                 })
//                 .unwrap(),
//             );
//             return vec![EngineMessage {
//                 topic,
//                 payload: "1".into(),
//                 qos: QoS::AtMostOnce,
//                 retain: false,
//             }];
//         }
//         //LightStatus set
//         let topic_set = topic.clone() + "/set";
//         if action.matches(&topic_set) {
//             let value = String::from_utf8_lossy(&action.payload);
//             mapinfo.insert(
//                 topic.clone(),
//                 bincode::serialize(&LightStatus {
//                     temp: None,
//                     value: value.to_string(),
//                 })
//                 .unwrap(),
//             );
//             return vec![EngineMessage {
//                 topic,
//                 payload: value.to_string().into(),
//                 qos: QoS::AtMostOnce,
//                 retain: false,
//             }];
//         }
//         //LightStatus switch
//         let topic_command = topic.clone() + "/command";
//         if action.matches(&topic_command) {
//             let value = String::from_utf8_lossy(&action.payload);
//             let status = get_light_status(mapinfo, &topic);
//             if value.eq("switch") {
//                 let newvalue: String = if status.value == "1" { "0" } else { "1" }.into();
//                 let newpayload: Vec<u8> = newvalue.clone().into();
//                 mapinfo.insert(
//                     topic.clone(),
//                     bincode::serialize(&LightStatus {
//                         temp: None,
//                         value: newvalue,
//                     })
//                     .unwrap(),
//                 );
//                 return vec![EngineMessage {
//                     topic,
//                     payload: newpayload,
//                     qos: QoS::AtMostOnce,
//                     retain: false,
//                 }];
//             }
//         }
//         // Timer for temporization
//         if action.matches("SYSMR/user_action/tick") {
//             let status = get_light_status(mapinfo, &topic);
//             // if temporizator activated and time consumed then switch off
//             if let Some(t) = status.temp {
//                 if action.timestamp > t {
//                     mapinfo.insert(
//                         topic.clone(),
//                         bincode::serialize(&LightStatus {
//                             temp: None,
//                             value: "0".to_string(),
//                         })
//                         .unwrap(),
//                     );
//                     return vec![EngineMessage {
//                         topic,
//                         payload: "0".into(),
//                         qos: QoS::AtMostOnce,
//                         retain: false,
//                     }];
//                 }
//             }
//         }
//         vec![]
//     }
// }

// pub fn modal_value(
//     strtopic: &str,
// ) -> impl FnOnce(&mut HashMap<String, Vec<u8>>, &ActionMessage) -> Vec<EngineMessage> {
//     let topic = strtopic.to_string();
//     let mut topic_value = strtopic.to_string();
//     topic_value.push_str("/value");
//     move |_: &mut HashMap<String, Vec<u8>>, action: &ActionMessage| -> Vec<EngineMessage> {
//         if action.matches(&topic_value) {
//             return vec![EngineMessage {
//                 topic,
//                 payload: "0".into(),
//                 qos: QoS::AtMostOnce,
//                 retain: false,
//             }];
//         }
//         vec![]
//     }
// }
