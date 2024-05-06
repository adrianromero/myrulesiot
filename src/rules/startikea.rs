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

use serde_json::json;

use super::startaction::imp_start_json_action;
use crate::master::{EngineAction, SliceFunction};
pub enum IkeaRemote {
    On,
    Off,
    Toggle,
    BrightUp,
    BrightDown,
    ArrowRight,
    ArrowLeft,
}

pub fn start_ikea_remote(command: IkeaRemote) -> SliceFunction {
    Box::new(move |info: &serde_json::Value, action: &EngineAction| {
        let topic = info["_topic"].as_str().unwrap();
        imp_start_json_action(
            info,
            action,
            topic,
            "/action",
            &json!(ikea_remote_to_string(&command)),
        )
    })
}

fn ikea_remote_to_string(r: &IkeaRemote) -> &'static str {
    match r {
        IkeaRemote::On => "on",
        IkeaRemote::Off => "off",
        IkeaRemote::Toggle => "toggle",
        IkeaRemote::BrightUp => "brightness_up_click",
        IkeaRemote::BrightDown => "brightness_down_click",
        IkeaRemote::ArrowRight => "arrow_right_click",
        IkeaRemote::ArrowLeft => "arrow_left_click",
    }
}
