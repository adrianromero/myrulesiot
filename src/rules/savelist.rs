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

use rumqttc::QoS;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::mqtt::{ActionMessage, ConnectionMessage};

#[derive(Serialize, Deserialize)]
struct ListStatus {
    temp: Option<i64>,
    current: Option<Vec<u8>>,
    values: Vec<Option<Vec<u8>>>,
}
impl Default for ListStatus {
    fn default() -> Self {
        ListStatus {
            temp: None,
            current: None,
            values: vec![],
        }
    }
}

fn values_to_string(values: &Vec<Option<Vec<u8>>>) -> String {
    let newvalues: Vec<String> = values
        .iter()
        .map(|value| match value {
            None => String::from("null"),
            Some(v) => String::from_utf8_lossy(&v).to_string(),
        })
        .collect();

    format!("[{}]", newvalues.join(","))
}

fn get_list_status(mapinfo: &mut HashMap<String, Vec<u8>>, topic: &str) -> ListStatus {
    mapinfo
        .get(topic)
        .map(|s| bincode::deserialize::<ListStatus>(s).unwrap())
        .unwrap_or_default()
}

pub fn save_list(
    strtopic: &str,
    time_period: &chrono::Duration,
    count_values: usize,
) -> impl FnOnce(&mut HashMap<String, Vec<u8>>, &ActionMessage) -> Vec<ConnectionMessage> {
    let topic = strtopic.to_string();
    let mut topic_store = strtopic.to_string();
    topic_store.push_str("/list");
    let mut topic_list = strtopic.to_string();
    topic_list.push_str("/list");
    let time_tick: i64 = time_period.num_milliseconds() / count_values as i64;
    move |mapinfo: &mut HashMap<String, Vec<u8>>,
          action: &ActionMessage|
          -> Vec<ConnectionMessage> {
        if action.matches(&topic) {
            let mut status = get_list_status(mapinfo, &topic_store);
            status.current = Some(action.payload.to_vec());
            mapinfo.insert(topic_store, bincode::serialize(&status).unwrap());
            return vec![];
        }

        // Timer for temporization
        if action.matches("SYSMR/user_action/tick") {
            let mut status = get_list_status(mapinfo, &topic_store);
            // if temporizator activated and time consumed then switch off
            match status.temp {
                None => {
                    status.temp = Some(action.timestamp);
                    status.values = vec![None; count_values];
                    if let Some(last) = status.values.last_mut() {
                        *last = status.current.clone();
                    }
                    mapinfo.insert(topic_store, bincode::serialize(&status).unwrap());
                    return vec![ConnectionMessage {
                        topic: topic_list,
                        payload: values_to_string(&status.values).into(),
                        qos: QoS::AtMostOnce,
                        retain: false,
                    }];
                }
                Some(t) => {
                    if action.timestamp > t + time_tick {
                        let mut newt = t;
                        while action.timestamp > newt + time_tick {
                            newt = newt + time_tick;
                            status.temp = Some(newt); // while because this also can be less than action.timestamp
                            status.values.rotate_left(1);
                            if let Some(last) = status.values.last_mut() {
                                *last = status.current.clone();
                            }
                        }
                        mapinfo.insert(topic_store, bincode::serialize(&status).unwrap());
                        return vec![ConnectionMessage {
                            topic: topic_list,
                            payload: values_to_string(&status.values).into(),
                            qos: QoS::AtMostOnce,
                            retain: false,
                        }];
                    }
                }
            }
        }
        vec![]
    }
}

pub fn save_value(
    strtopic: &str,
) -> impl FnOnce(&mut HashMap<String, Vec<u8>>, &ActionMessage) -> Vec<ConnectionMessage> {
    let topic = strtopic.to_string();
    let mut topic_store = strtopic.to_string();
    topic_store.push_str("/store");
    move |mapinfo: &mut HashMap<String, Vec<u8>>,
          action: &ActionMessage|
          -> Vec<ConnectionMessage> {
        if action.matches(&topic) {
            mapinfo.insert(topic_store, action.payload.to_vec());
        }
        vec![]
    }
}
