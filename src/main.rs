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

use rumqttc::{self, QoS};
use std::error::Error;
use tokio::join;
use tokio::sync::mpsc;
use tokio::task;

mod mqtt;
use mqtt::{ConnectionInfo, ConnectionMessage, ConnectionResult};

mod engine;

mod mainengine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    log::info!("Starting myrulesiot...");

    // Defines connection properties
    let connection_info = ConnectionInfo {
        id: "rustclient-231483".into(),
        host: "localhost".into(),
        clean_session: true,
        ..Default::default()
    };

    let (sub_tx, sub_rx) = mpsc::channel::<ConnectionMessage>(10);
    let (pub_tx, pub_rx) = mpsc::channel::<ConnectionResult>(10);

    let (client, eventloop) = mqtt::new_connection(
        connection_info,
        vec![(String::from("myhelloiot/#"), QoS::AtMostOnce)],
    )
    .await?;

    // Engine definition
    let engine = mainengine::create_main_engine();

    // Runtime things
    let j1 = task::spawn(async move {
        match engine::runtime_loop(engine, pub_tx, sub_rx).await {
            Result::Ok(_) => {}
            Result::Err(error) => {
                log::warn!("Runtime error {}", error);
            }
        }
        log::info!("Exiting spawn runtime engine...");
    });

    // MQTT things
    let j2 = task::spawn(async move {
        mqtt::publication_loop(client, pub_rx).await;
        log::info!("Exiting spawn mqtt publication...");
    });
    let j3 = task::spawn(async move {
        match mqtt::subscription_loop(eventloop, sub_tx).await {
            Result::Ok(_) => {}
            Result::Err(error) => {
                log::warn!("Connection error {}", error);
            }
        }
        log::info!("Exiting spawn mqtt subscription...");
    });

    let (_, _, _) = join!(j1, j2, j3);

    log::info!("Exiting myrulesiot...");
    Ok(())
}
