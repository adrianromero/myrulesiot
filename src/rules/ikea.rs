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

use super::actionactuator::imp_actuator_json_action;
use crate::mqtt::{EngineAction, SliceFunction};
pub enum IkeaRemote {
    Toggle,
    BrightUp,
    BrightDown,
    ArrowRight,
    ArrowDown,
}

pub fn actuator_ikea_remote(command: IkeaRemote) -> SliceFunction {
    Box::new(
        move |params: &serde_json::Value, info: &serde_json::Value, action: &EngineAction| {
            let topic = params["topic"].as_str().unwrap();
            imp_actuator_json_action(
                info,
                action,
                topic,
                "/action",
                &json!(ikea_remote_to_string(&command)),
            )
        },
    )
}

fn ikea_remote_to_string(r: &IkeaRemote) -> &'static str {
    match r {
        IkeaRemote::Toggle => "toggle",
        IkeaRemote::BrightUp => "brightness_up_click",
        IkeaRemote::BrightDown => "brightness_down_click",
        IkeaRemote::ArrowRight => "arrow_right_click",
        IkeaRemote::ArrowDown => "arrow_left_click",
    }
}
