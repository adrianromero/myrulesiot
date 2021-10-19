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

use crate::mqtt::ConnectionMessage;

#[derive(Serialize, Deserialize)]
struct Temporizator(i32);
impl Default for Temporizator {
    fn default() -> Self {
        Temporizator(-1)
    }
}

pub fn light_temp(
    strtopic: &str,
) -> impl FnOnce(&mut HashMap<String, Vec<u8>>, &ConnectionMessage) -> Vec<ConnectionMessage> {
    let topic = strtopic.to_string();
    let mut topic_set = strtopic.to_string();
    topic_set.push_str("/set");

    move |mapinfo: &mut HashMap<String, Vec<u8>>,
          action: &ConnectionMessage|
          -> Vec<ConnectionMessage> {
        if action.matches(&topic_set) {
            let smillis = String::from_utf8_lossy(&action.payload);
            let millis: i32 = smillis.parse().unwrap_or(5000) / 250;
            mapinfo.insert(
                topic.clone(),
                bincode::serialize(&Temporizator(millis)).unwrap(),
            );
            return vec![ConnectionMessage {
                topic,
                qos: QoS::AtMostOnce,
                retain: false,
                payload: "1".into(),
            }];
        }
        if action.matches(
            "SYSMR/timer
        ",
        ) {
            let t = mapinfo
                .get(&topic)
                .map(|s| bincode::deserialize::<Temporizator>(s).unwrap())
                .unwrap_or(Temporizator::default());
            let counter = t.0;
            if counter > 0 {
                mapinfo.insert(
                    topic,
                    bincode::serialize(&Temporizator(counter - 1)).unwrap(),
                );
            } else if counter == 0 {
                mapinfo.insert(
                    topic.clone(),
                    bincode::serialize(&Temporizator::default()).unwrap(),
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
