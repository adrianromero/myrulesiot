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

use tokio::{task, time};

use rumqttc::{self, AsyncClient, MqttOptions, QoS};
use std::error::Error;
use std::time::Duration;

mod mqtt;
use mqtt::ConnectionInfo;

mod engine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // log::trace!("Executing query: {}", "pepe");
    // log::debug!("Executing query: {}", "debug");
    // log::info!("Executing info.");
    // log::warn!("Executing warn.");
    // log::error!("Executing error.");

    // Defines connection properties
    let connection_info = ConnectionInfo {
        id: "rustclient-231483".into(),
        host: "localhost".into(),
        clean_session: true,
        ..Default::default()
    };

    connectivity(connection_info).await?;

    println!("Salimos finos...");
    Ok(())
}

async fn connectivity(connection_info: ConnectionInfo) -> Result<(), Box<dyn Error>> {
    // Connects to the MQTT server
    let mut mqttoptions = MqttOptions::new(
        connection_info.id,
        connection_info.host,
        connection_info.port,
    );
    mqttoptions
        .set_keep_alive(connection_info.keep_alive)
        .set_inflight(connection_info.inflight)
        .set_clean_session(connection_info.clean_session);

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe("myhelloiot/modal", QoS::AtMostOnce)
        .await?;

    task::spawn(async move {
        requests(client).await;
        time::sleep(Duration::from_secs(3)).await;
    });

    // See example in https://github.com/bytebeamio/rumqtt/issues/263
    // See channel use https://docs.rs/tokio/1.12.0/tokio/sync/mpsc/struct.Sender.html
    loop {
        let result = eventloop.poll().await;
        match result {
            Result::Ok(event) => {
                println!("Event -> {:?}", event);
            }
            Result::Err(rumqttc::ConnectionError::Cancel) => {
                println!("Notify -> Cancel");
                break;
            }
            Result::Err(error) => {
                println!("Error -> {:?}", error);
                Result::Err(error)?;
            }
        }
    }

    Result::Ok(())
}

async fn requests(client: AsyncClient) {
    // for i in 1..=10 {
    //     client
    //         .publish("hello/world", QoS::ExactlyOnce, false, vec![1; i])
    //         .await
    //         .unwrap();

    //     time::sleep(Duration::from_secs(1)).await;
    // }

    time::sleep(Duration::from_secs(10)).await;
    client.cancel().await.unwrap();
    //client.disconnect().await.unwrap();
}
