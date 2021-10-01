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

use rumqttc::{self, AsyncClient, Publish, QoS};
use std::error::Error;
use std::str;
use std::time::Duration;
use tokio::sync::mpsc;

mod mqtt;
use mqtt::{Connection, ConnectionInfo};

// mod engine;

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

    let (client, eventloop) = Connection::new(
        connection_info,
        vec![(String::from("myhelloiot/modal"), QoS::AtMostOnce)],
    )
    .await?;

    let (tx, mut rx) = mpsc::channel::<Publish>(10);

    let j1 = tokio::task::spawn(async move {
        requests(client).await;
        tokio::time::sleep(Duration::from_secs(3)).await;
        log::info!("Exiting spawn requests...");
    });

    let j2 = tokio::task::spawn(async move {
        while let Some(message) = rx.recv().await {
            let payload = str::from_utf8(&message.payload).unwrap_or("<ERROR>");
            log::info!("Receiving: Topic = {}, Payload ={}", message.topic, payload);
        }
        log::info!("Exiting spawn receiver...");
    });

    Connection::do_loop(eventloop, tx).await?;

    let (_, _) = tokio::join!(j1, j2);

    log::info!("Exiting myrulesiot...");
    Ok(())
}

async fn requests(client: AsyncClient) {
    for i in 1..=10 {
        client
            .publish(
                "myhelloiot/modal",
                QoS::ExactlyOnce,
                false,
                i.to_string().as_bytes(),
            )
            .await
            .unwrap();

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    tokio::time::sleep(Duration::from_secs(1)).await;
    client.cancel().await.unwrap();
    //client.disconnect().await.unwrap();
}
