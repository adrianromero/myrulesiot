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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use rumqttc::QoS;

use crate::mqtt::{ConnectionAction, ConnectionMessage};

#[derive(Serialize, Deserialize)]
struct LightStatus {
    pub temp: Option<i64>,
    pub value: String,
}

impl Default for LightStatus {
    fn default() -> Self {
        LightStatus {
            temp: None,
            value: "0".to_string(),
        }
    }
}

fn get_light_status(mapinfo: &mut HashMap<String, Vec<u8>>, topic: &str) -> LightStatus {
    mapinfo
        .get(&format!("light_{}", topic))
        .map(|s| bincode::deserialize::<LightStatus>(s).unwrap())
        .unwrap_or_default()
}

fn insert_light_status(mapinfo: &mut HashMap<String, Vec<u8>>, topic: &str, status: &LightStatus) {
    mapinfo.insert(
        format!("light_{}", topic),
        bincode::serialize(status).unwrap(),
    );
}

pub fn toggle(
    actionmatch: impl Fn(&ConnectionAction) -> bool,
    strtopic: impl Into<String>,
    strtopicpub: impl Into<String>,
) -> impl Fn(&mut HashMap<String, Vec<u8>>, &ConnectionAction) -> Vec<ConnectionMessage> {
    let topic = strtopic.into();
    let topicpub = strtopicpub.into();
    move |mapinfo: &mut HashMap<String, Vec<u8>>,
          action: &ConnectionAction|
          -> Vec<ConnectionMessage> {
        if actionmatch(action) {
            let status = get_light_status(mapinfo, &topic);
            let newvalue: String = if status.value == "1" { "0" } else { "1" }.into();
            let newpayload: Vec<u8> = newvalue.clone().into();
            insert_light_status(
                mapinfo,
                &topic,
                &LightStatus {
                    temp: None,
                    value: newvalue,
                },
            );
            return vec![ConnectionMessage {
                topic: topicpub.clone(),
                payload: newpayload,
                qos: QoS::AtMostOnce,
                retain: false,
            }];
        }

        vec![]
    }
}

pub fn light_set(
    actionmatch: impl Fn(&ConnectionAction) -> bool,
    strtopic: impl Into<String>,
    strtopicpub: impl Into<String>,
    strvalue: impl Into<String>,
) -> impl Fn(&mut HashMap<String, Vec<u8>>, &ConnectionAction) -> Vec<ConnectionMessage> {
    let topic = strtopic.into();
    let topicpub = strtopicpub.into();
    let newvalue = strvalue.into();
    move |mapinfo: &mut HashMap<String, Vec<u8>>,
          action: &ConnectionAction|
          -> Vec<ConnectionMessage> {
        if actionmatch(action) {
            let newpayload: Vec<u8> = newvalue.clone().into();
            insert_light_status(
                mapinfo,
                &topic,
                &LightStatus {
                    temp: None,
                    value: newvalue.clone(),
                },
            );
            return vec![ConnectionMessage {
                topic: topicpub.clone(),
                payload: newpayload,
                qos: QoS::AtMostOnce,
                retain: false,
            }];
        }

        vec![]
    }
}

pub fn light_on(
    actionmatch: impl Fn(&ConnectionAction) -> bool,
    strtopic: impl Into<String>,
    strtopicpub: impl Into<String>,
) -> impl Fn(&mut HashMap<String, Vec<u8>>, &ConnectionAction) -> Vec<ConnectionMessage> {
    light_set(actionmatch, strtopic, strtopicpub, "1")
}

pub fn light_off(
    actionmatch: impl Fn(&ConnectionAction) -> bool,
    strtopic: impl Into<String>,
    strtopicpub: impl Into<String>,
) -> impl Fn(&mut HashMap<String, Vec<u8>>, &ConnectionAction) -> Vec<ConnectionMessage> {
    light_set(actionmatch, strtopic, strtopicpub, "0")
}

pub fn light_time(
    actionmatch: impl Fn(&ConnectionAction) -> bool,
    strtopic: impl Into<String>,
    strtopicpub: impl Into<String>,
) -> impl Fn(&mut HashMap<String, Vec<u8>>, &ConnectionAction) -> Vec<ConnectionMessage> {
    let topic = strtopic.into();
    let topicpub = strtopicpub.into();
    move |mapinfo: &mut HashMap<String, Vec<u8>>,
          action: &ConnectionAction|
          -> Vec<ConnectionMessage> {
        if actionmatch(action) {
            // let smillis = String::from_utf8_lossy(&action.payload);
            // let millis: i64 = smillis.parse().unwrap_or(5000);
            let millis = 5000i64;
            insert_light_status(
                mapinfo,
                &topic,
                &LightStatus {
                    temp: Some(action.timestamp + millis),
                    value: String::from("1"),
                },
            );
            return vec![ConnectionMessage {
                topic: topicpub.clone(),
                payload: vec![b'1'],
                qos: QoS::AtMostOnce,
                retain: false,
            }];
        }

        vec![]
    }
}

pub fn light_time_reset(
    strtopic: impl Into<String>,
    strtopicpub: impl Into<String>,
) -> impl Fn(&mut HashMap<String, Vec<u8>>, &ConnectionAction) -> Vec<ConnectionMessage> {
    let topic = strtopic.into();
    let topicpub = strtopicpub.into();
    move |mapinfo: &mut HashMap<String, Vec<u8>>,
          action: &ConnectionAction|
          -> Vec<ConnectionMessage> {
        if action.matches("SYSMR/user_action/tick") {
            let status = get_light_status(mapinfo, &topic);
            // if temporizator activated and time consumed then switch off
            if let Some(t) = status.temp {
                if action.timestamp > t {
                    insert_light_status(
                        mapinfo,
                        &topic,
                        &LightStatus {
                            temp: None,
                            value: "0".to_string(),
                        },
                    );
                    return vec![ConnectionMessage {
                        topic: topicpub.clone(),
                        payload: vec![b'0'],
                        qos: QoS::AtMostOnce,
                        retain: false,
                    }];
                }
            }
        }

        vec![]
    }
}

pub fn status(
    strtopic: impl Into<String>,
) -> impl Fn(&mut HashMap<String, Vec<u8>>, &ConnectionAction) -> Vec<ConnectionMessage> {
    let topic = strtopic.into();
    move |mapinfo: &mut HashMap<String, Vec<u8>>,
          action: &ConnectionAction|
          -> Vec<ConnectionMessage> {
        if action.matches(&topic) {
            let status = get_light_status(mapinfo, &topic);
            let value = String::from_utf8_lossy(&action.payload);
            insert_light_status(
                mapinfo,
                &topic,
                &LightStatus {
                    temp: if value == "0" { None } else { status.temp },
                    value: value.to_string(),
                },
            );
        }

        vec![]
    }
}
