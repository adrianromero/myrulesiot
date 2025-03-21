//    MyRulesIoT  Project is a rules engine for MQTT based on MyRulesIoT lib
//    Copyright (C) 2022-2025 Adrián Romero Corchado.
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

use config::Config;
use myrulesiot::master::EngineStatus;
use myrulesiot::master::FinalStatus;
use std::error::Error;
use std::fs;
use std::path::Path;

use tokio::sync::mpsc;
use tokio::{task, try_join};

use myrulesiot::master::{
    self, EngineAction, EngineResult, EngineState, MasterEngine, ReducerFunction,
};
use myrulesiot::mqtt::{self, ConnectionValues, Subscription};
use myrulesiot::rules;
use myrulesiot::runtime;

const FUNCTIONS_PATH: &str = "./engine_functions.json";
const EXIT_PATH: &str = "./engine_exit";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Settings
    let settings = Config::builder()
        .add_source(config::File::with_name("./homerules"))
        .add_source(config::Environment::with_prefix("HOMERULES"))
        .build()?;

    let prefix_id = settings
        .get_string("application.identifier")
        .unwrap_or_else(|_| String::from("HOMERULES"));

    // Exit
    fs::remove_file(EXIT_PATH).unwrap_or_default();

    // Functions
    let functions = match Path::new(FUNCTIONS_PATH).try_exists() {
        Ok(true) => {
            let f = fs::read(FUNCTIONS_PATH).map_err(|error| {
                format!("Cannot read ReducerFunctions file {FUNCTIONS_PATH}: {error}")
            })?;
            serde_json::from_slice::<Vec<ReducerFunction>>(&f).map_err(|error| {
                format!("Cannot parse JSON ReducerFunctions file {FUNCTIONS_PATH}: {error}")
            })?
        }
        _ => Vec::new(),
    };

    let (sub_tx, sub_rx) = mpsc::channel::<EngineAction>(10);
    let (pub_tx, pub_rx) = mpsc::channel::<EngineResult>(10);

    // MQTT Connection
    let connection_info: ConnectionValues = settings.get::<ConnectionValues>("mqtt.connection")?;
    let mut subscriptions: Vec<Subscription> = settings
        .get::<Vec<Subscription>>("mqtt.subscriptions")
        .unwrap_or(vec![]);
    subscriptions.push(Subscription {
        topic: format!("{prefix_id}/command/#"),
        qos: 0,
    });

    log::info!("Connecting to MQTT broker: {:?}", &connection_info);
    let (client, eventloop) = mqtt::new_connection(connection_info, subscriptions)
        .await
        .map_err(|error| format!("MQTT error: {error}"))?;

    // MQTT
    let mqttsubscribetask = mqtt::task_subscription_loop(sub_tx.clone(), eventloop);
    let mqttpublishtask = mqtt::task_publication_loop(pub_rx, client);

    // Senders of EngineAction's
    let timertask = master::task_timer_loop(sub_tx.clone(), chrono::Duration::milliseconds(250));

    // THE RUNTIME ENGINE
    let enginetask = runtime::task_runtime_loop(
        pub_tx.clone(),
        sub_rx,
        MasterEngine::new(prefix_id, rules::distributed_engine_functions()),
        EngineState::new_functions(functions),
    );

    std::mem::drop(sub_tx);
    std::mem::drop(pub_tx);

    log::info!("Starting myrulesiot...");
    let (state, _, _, _) = try_join!(
        task::spawn(enginetask),
        task::spawn(timertask),
        task::spawn(mqttsubscribetask),
        task::spawn(mqttpublishtask)
    )?;
    log::info!("Exiting myrulesiot...");

    fs::write(
        FUNCTIONS_PATH,
        serde_json::to_vec_pretty(&state.functions).unwrap(),
    )?;

    match state.engine_status {
        EngineStatus::FINAL(status, message) => {
            let status_string = match status {
                FinalStatus::NORMAL => "NORMAL",
                FinalStatus::ERROR => "ERROR",
            };
            fs::write(EXIT_PATH, format!("{status_string}/{message}"))?;
            Ok(())
        }
        _ => Err("EngineStatus is not FINAL"),
    }?;

    Ok(())
}
