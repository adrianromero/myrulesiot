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

use rumqttc::{AsyncClient, ClientError, EventLoop, QoS};
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::try_join;

mod mqtt;
use mqtt::{ActionMessage, ConnectionInfo, ConnectionMessage, ConnectionResult, ConnectionState};
mod engine;
mod rules;
mod timer;

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

fn app_map_reducers(
) -> Vec<Box<dyn FnOnce(&mut HashMap<String, Vec<u8>>, &ActionMessage) -> Vec<ConnectionMessage>>> {
    vec![
        Box::new(rules::light_actions("myhelloiot/light1")),
        Box::new(rules::forward_timer("myhelloiot/timer")),
        Box::new(rules::modal_value("myhelloiot/alarm")),
        Box::new(rules::save_value("myhelloiot/temperature")),
        Box::new(rules::save_list(
            "myhelloiot/temperature",
            &chrono::Duration::minutes(1),
            20,
        )),
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

async fn connect_mqtt() -> Result<(AsyncClient, EventLoop), ClientError> {
    // Defines connection properties
    let connection_info = ConnectionInfo {
        id: "rustclient-231483".into(),
        host: "localhost".into(),
        clean_session: true,
        ..Default::default()
    };
    let subscriptions = &[
        ("myhelloiot/#", QoS::AtMostOnce),
        ("SYSMR/system_action", QoS::AtMostOnce),
    ];

    mqtt::new_connection(connection_info, subscriptions).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    log::info!("Starting myrulesiot...");
    let (client, eventloop) = connect_mqtt().await?;

    let (sub_tx, sub_rx) = mpsc::channel::<ActionMessage>(10);
    let (pub_tx, pub_rx) = broadcast::channel::<ConnectionResult>(10);

    let timertask = timer::task_timer_loop(&sub_tx, &chrono::Duration::milliseconds(250));
    let mqttsubscribetask = mqtt::task_subscription_loop(&sub_tx, eventloop);
    let mqttpublishtask = mqtt::task_publication_loop(pub_rx, client); // or pub_tx.subscribe()

    let engine = mqtt::create_engine(app_reducer);
    let enginetask = engine::task_runtime_loop(pub_tx, sub_rx, engine);

    let _ = try_join!(enginetask, mqttpublishtask, mqttsubscribetask, timertask)?;

    log::info!("Exiting myrulesiot...");
    Ok(())
}
