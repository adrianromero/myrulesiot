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
use std::error::Error;

use tokio::sync::mpsc;
use tokio::try_join;

mod configuration;
mod mqtt;
use mqtt::{ActionMessage, ConnectionMessage, ConnectionResult, ConnectionState};
mod devices;
mod engine;
mod rules;
mod timer;

use rules::lights;
use rules::zigbee;

#[derive(Debug, Clone)]
struct AppInfo {
    map: HashMap<String, Vec<u8>>,
}

impl Default for AppInfo {
    fn default() -> Self {
        AppInfo {
            map: HashMap::new(),
        }
    }
}

fn app_final(_: &AppInfo, action: &ActionMessage) -> bool {
    action.matches_action("SYSMR/system_action", "exit".into())
}

type FnReducer =
    dyn FnOnce(&mut HashMap<String, Vec<u8>>, &ActionMessage) -> Vec<ConnectionMessage>;

type ReducersVec = Vec<Box<FnReducer>>;

fn app_map_reducers() -> ReducersVec {
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

fn app_reducer(state: ConnectionState<AppInfo>, action: ActionMessage) -> ConnectionState<AppInfo> {
    let mut messages = Vec::<ConnectionMessage>::new();
    let mut newmap = state.info.map.clone();

    for f in app_map_reducers() {
        messages.append(&mut f(&mut newmap, &action));
    }

    let is_final = app_final(&state.info, &action);

    ConnectionState {
        info: AppInfo { map: newmap },
        messages,
        is_final,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    log::info!("Starting myrulesiot...");
    let (client, eventloop) = configuration::connect_mqtt().await?;

    let (sub_tx, sub_rx) = mpsc::channel::<ActionMessage>(10);
    let (pub_tx, pub_rx) = mpsc::channel::<ConnectionResult>(10);

    let timertask = timer::task_timer_loop(&sub_tx, &chrono::Duration::milliseconds(250));
    let mqttsubscribetask = mqtt::task_subscription_loop(&sub_tx, eventloop);
    let mqttpublishtask = mqtt::task_publication_loop(pub_rx, client); // or pub_tx.subscribe() if broadcast

    let engine = mqtt::create_engine(app_reducer);
    let enginetask = engine::task_runtime_loop(pub_tx, sub_rx, engine);

    let _ = try_join!(enginetask, mqttpublishtask, mqttsubscribetask, timertask)?;

    log::info!("Exiting myrulesiot...");
    Ok(())
}
