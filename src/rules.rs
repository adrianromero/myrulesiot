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

use crate::master::SliceFunction;

use self::startikea::IkeaRemote;

pub mod forward;
pub mod relay;
pub mod savelist;
pub mod startaction;
pub mod startikea;
pub mod timing;

pub fn default_engine_functions() -> HashMap<String, SliceFunction> {
    HashMap::from([
        (String::from("start_action"), startaction::start_action()),
        (
            String::from("start_json_action"),
            startaction::start_json_action(),
        ),
        (
            String::from("start_ikea_remote_toggle"),
            startikea::start_ikea_remote(IkeaRemote::Toggle),
        ),
        (
            String::from("start_ikea_remote_bright_down"),
            startikea::start_ikea_remote(IkeaRemote::BrightDown),
        ),
        (String::from("relay_on"), relay::relay_value(b"on")),
        (String::from("relay"), relay::relay()),
        (String::from("forward_action"), forward::forward_action()),
        (
            String::from("forward_user_action"),
            forward::forward_user_action(),
        ),
        (String::from("condition_sleep"), timing::condition_sleep()),
    ])
}
