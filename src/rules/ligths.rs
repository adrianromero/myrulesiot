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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct LightStatus {
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

pub fn get_light_status(mapinfo: &mut HashMap<String, Vec<u8>>, topic: &str) -> LightStatus {
    mapinfo
        .get(topic)
        .map(|s| bincode::deserialize::<LightStatus>(s).unwrap())
        .unwrap_or_default()
}
