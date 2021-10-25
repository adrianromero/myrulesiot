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

use bytes::Bytes;
use rumqttc::QoS;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::mqtt::{ActionMessage, ConnectionMessage};

#[derive(Serialize, Deserialize)]
struct LightStatus {
    temp: i32,
    value: String,
}
impl Default for LightStatus {
    fn default() -> Self {
        LightStatus {
            temp: -1,
            value: "0".to_string(),
        }
    }
}

fn get_light_status(mapinfo: &mut HashMap<String, Vec<u8>>, topic: &str) -> LightStatus {
    let status = mapinfo
        .get(topic)
        .map(|s| bincode::deserialize::<LightStatus>(s).unwrap())
        .unwrap_or(LightStatus::default());
    status
}

pub fn light_temp(
    strtopic: &str,
) -> impl FnOnce(&mut HashMap<String, Vec<u8>>, &ActionMessage) -> Vec<ConnectionMessage> {
    let topic = strtopic.to_string();

    move |mapinfo: &mut HashMap<String, Vec<u8>>,
          action: &ActionMessage|
          -> Vec<ConnectionMessage> {
        //LightStatus temporizator
        let topic_temp = topic.clone() + "/temp";
        if action.matches(&topic_temp) {
            let smillis = String::from_utf8_lossy(&action.payload);
            let millis: i32 = smillis.parse().unwrap_or(5000) / 250;
            mapinfo.insert(
                topic.clone(),
                bincode::serialize(&LightStatus {
                    temp: millis,
                    value: "1".to_string(),
                })
                .unwrap(),
            );
            return vec![ConnectionMessage {
                topic,
                qos: QoS::AtMostOnce,
                retain: false,
                payload: "1".into(),
            }];
        }

        //LightStatus set
        let topic_set = topic.clone() + "/set";
        if action.matches(&topic_set) {
            let value = String::from_utf8_lossy(&action.payload);
            mapinfo.insert(
                topic.clone(),
                bincode::serialize(&LightStatus {
                    temp: -1,
                    value: value.to_string(),
                })
                .unwrap(),
            );
            return vec![ConnectionMessage {
                topic,
                qos: QoS::AtMostOnce,
                retain: false,
                payload: value.to_string().into(),
            }];
        }

        //LightStatus switch
        let topic_command = topic.clone() + "/command";
        if action.matches(&topic_command) {
            let value = String::from_utf8_lossy(&action.payload);
            let status = get_light_status(mapinfo, &topic);
            if value.eq("switch") {
                let newvalue: String = if status.value == "1" { "0" } else { "1" }.into();
                let newpayload: Bytes = newvalue.clone().into();
                mapinfo.insert(
                    topic.clone(),
                    bincode::serialize(&LightStatus {
                        temp: -1,
                        value: newvalue,
                    })
                    .unwrap(),
                );
                return vec![ConnectionMessage {
                    topic,
                    qos: QoS::AtMostOnce,
                    retain: false,
                    payload: newpayload,
                }];
            }
        }

        // Timer for temporization
        if action.matches("SYSMR/timer") {
            let status = get_light_status(mapinfo, &topic);
            let counter = status.temp;
            if counter > 0 {
                mapinfo.insert(
                    topic.clone(),
                    bincode::serialize(&LightStatus {
                        temp: counter - 1,
                        ..status
                    })
                    .unwrap(),
                );
            } else if counter == 0 {
                mapinfo.insert(
                    topic.clone(),
                    bincode::serialize(&LightStatus {
                        temp: -1,
                        value: "0".to_string(),
                    })
                    .unwrap(),
                );
                return vec![ConnectionMessage {
                    topic,
                    qos: QoS::AtMostOnce,
                    retain: false,
                    payload: "0".into(),
                }];
            }
        }

        vec![]
    }
}

pub fn modal_value(
    strtopic: &str,
) -> impl FnOnce(&mut HashMap<String, Vec<u8>>, &ActionMessage) -> Vec<ConnectionMessage> {
    let topic = strtopic.to_string();
    let mut topic_value = strtopic.to_string();
    topic_value.push_str("/value");
    move |_: &mut HashMap<String, Vec<u8>>, action: &ActionMessage| -> Vec<ConnectionMessage> {
        if action.matches(&topic_value) {
            return vec![ConnectionMessage {
                topic,
                qos: QoS::AtMostOnce,
                retain: false,
                payload: "0".into(),
            }];
        }
        vec![]
    }
}

pub fn forward_timer(
    strtopic: &str,
) -> impl FnOnce(&mut HashMap<String, Vec<u8>>, &ActionMessage) -> Vec<ConnectionMessage> {
    let topic = strtopic.to_string();
    move |_: &mut HashMap<String, Vec<u8>>, action: &ActionMessage| -> Vec<ConnectionMessage> {
        if action.matches("SYSMR/timer") {
            return vec![ConnectionMessage {
                topic,
                qos: QoS::AtMostOnce,
                retain: false,
                payload: action.payload.clone(),
            }];
        }
        vec![]
    }
}
