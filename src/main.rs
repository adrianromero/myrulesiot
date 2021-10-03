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
use tokio::sync::mpsc;

mod mqtt;
use mqtt::{Connection, ConnectionInfo, ConnectionResult};

mod engine;
use engine::Engine;
use engine::RuntimeEngine;

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

    let (sub_tx, sub_rx) = mpsc::channel::<Publish>(10);
    let (pub_tx, pub_rx) = mpsc::channel::<ConnectionResult>(10);

    let (client, eventloop) = Connection::new(
        connection_info,
        vec![(String::from("myhelloiot/modal"), QoS::AtMostOnce)],
    )
    .await?;

    async fn publishing(client: AsyncClient, mut pub_rx: mpsc::Receiver<ConnectionResult>) {
        // This is the future in charge of publishing result messages and canceling if final
        while let Some(res) = pub_rx.recv().await {
            for elem in res.messages.into_iter() {
                client
                    .publish(
                        elem.topic,
                        elem.qos,
                        elem.retain,
                        Vec::from(&elem.payload[..]),
                    )
                    .await
                    .unwrap();
            }

            if res.is_final {
                client.cancel().await.unwrap();
            }
        }
    }

    // Engine definition
    fn reduce(state: &u32, _action: &Publish) -> u32 {
        state + 1
    }

    fn template(state: &u32) -> ConnectionResult {
        ConnectionResult {
            messages: vec![],
            is_final: state == &4,
        }
    }

    let engine: Engine<Publish, ConnectionResult, u32> = Engine {
        reduce,
        template,
        is_final: |result: &ConnectionResult| result.is_final,
    };

    // Runtime things
    let j2 = tokio::task::spawn(async move {
        match RuntimeEngine::do_loop(engine, pub_tx, sub_rx).await {
            Result::Ok(_) => {}
            Result::Err(error) => {
                log::warn!("Runtime error {}", error);
            }
        }
        log::info!("Exiting spawn engine...");
    });

    // MQTT things
    let j1 = tokio::task::spawn(async move {
        publishing(client, pub_rx).await;
        log::info!("Exiting spawn mqtt publishing...");
    });
    let j3 = tokio::task::spawn(async move {
        match Connection::do_loop(eventloop, sub_tx).await {
            Result::Ok(_) => {}
            Result::Err(error) => {
                log::warn!("Connection error {}", error);
            }
        }
        log::info!("Exiting spawn mqtt subscription...");
    });
    let (_, _, _) = tokio::join!(j1, j2, j3);

    log::info!("Exiting myrulesiot...");
    Ok(())
}
