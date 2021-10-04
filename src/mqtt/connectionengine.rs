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

use std::error::Error;
use std::fmt::Debug;
use tokio::join;
use tokio::sync::mpsc;
use tokio::task;

use super::{new_connection, publication_loop, subscription_loop};
use super::{ConnectionInfo, ConnectionMessage, ConnectionResult, TopicInfo};
use crate::engine;

pub async fn connection_engine<T, S>(
    engine: engine::Engine<ConnectionMessage, ConnectionResult, T>,
    connection_info: ConnectionInfo,
    subscriptions: &[TopicInfo<S>],
) -> Result<(), Box<dyn Error>>
where
    T: Debug + Default + Send + 'static,
    S: Into<String> + Copy,
{
    log::info!("Starting myrulesiot...");

    let (sub_tx, sub_rx) = mpsc::channel::<ConnectionMessage>(connection_info.cap);
    let (pub_tx, pub_rx) = mpsc::channel::<ConnectionResult>(connection_info.cap);

    let (client, eventloop) = new_connection(connection_info, subscriptions).await?;

    // Runtime things
    let enginetask = task::spawn(async move {
        match engine::runtime_loop(engine, pub_tx, sub_rx).await {
            Result::Ok(_) => {}
            Result::Err(error) => {
                log::warn!("Runtime error {}", error);
            }
        }
        log::info!("Exiting spawn runtime engine...");
    });

    // MQTT things
    let mqttpublishtask = task::spawn(async move {
        publication_loop(client, pub_rx).await;
        log::info!("Exiting spawn mqtt publication...");
    });
    let mqttsubscribetask = task::spawn(async move {
        match subscription_loop(eventloop, sub_tx).await {
            Result::Ok(_) => {}
            Result::Err(error) => {
                log::warn!("Connection error {}", error);
            }
        }
        log::info!("Exiting spawn mqtt subscription...");
    });

    let (_, _, _) = join!(enginetask, mqttpublishtask, mqttsubscribetask);

    log::info!("Exiting myrulesiot...");
    Ok(())
}
