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

use rumqttc::{AsyncClient, ClientError, EventLoop, QoS};
use std::collections::HashMap;

use crate::mqtt;

use crate::devices;
use crate::lights;
use crate::rules;
use crate::zigbee;

pub async fn connect_mqtt() -> Result<(AsyncClient, EventLoop), ClientError> {
    // Defines connection properties
    let connection_info = mqtt::ConnectionInfo {
        id: "rustclient-231483".into(),
        host: "localhost".into(),
        clean_session: true,
        ..Default::default()
    };
    let subscriptions = &[
        ("myhelloiot/#", QoS::AtMostOnce),
        ("zigbee2mqtt/0x000b57fffe4fc5ca", QoS::AtMostOnce),
        ("SYSMR/system_action", QoS::AtMostOnce),
        ("ESPURNITA04/#", QoS::AtMostOnce),
    ];

    mqtt::new_connection(connection_info, subscriptions).await
}

type FnReducer =
    dyn FnOnce(&mut HashMap<String, Vec<u8>>, &mqtt::ActionMessage) -> Vec<mqtt::ConnectionMessage>;

type ReducersVec = Vec<Box<FnReducer>>;

pub fn app_map_reducers() -> ReducersVec {
    vec![
        Box::new(rules::save_value("SYSMR/user_action/tick")),
        Box::new(rules::forward_user_action_tick("myhelloiot/timer")),
        // Box::new(rules::light_actions("myhelloiot/light1")),
        // Box::new(rules::modal_value("myhelloiot/alarm")),
        Box::new(rules::save_list(
            "myhelloiot/temperature",
            &chrono::Duration::seconds(20),
            40,
        )),
        // Box::new(rules::forward_action(
        //     "zigbee2mqtt/0x000b57fffe4fc5ca",
        //     "ESPURNA04/relay/0/set",
        // )),
        Box::new(lights::toggle(
            zigbee::actuator_toggle("zigbee2mqtt/0x000b57fffe4fc5ca"),
            "ESPURNITA04/relay/0",
            "ESPURNITA04/relay/0/set",
        )),
        Box::new(lights::status("ESPURNITA04/relay/0")),
        Box::new(devices::simulate_relay("ESPURNITA04/relay/0")),
    ]
}
