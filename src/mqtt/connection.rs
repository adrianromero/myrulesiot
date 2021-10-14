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

use rumqttc::{self, AsyncClient, ConnectionError, Event, EventLoop, MqttOptions, Packet, QoS};
use std::error::Error;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::task;

use super::{ConnectionMessage, ConnectionResult};

#[derive(Debug)]
pub struct ConnectionInfo {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub keep_alive: u16,
    pub inflight: u16,
    pub clean_session: bool,
    pub cap: usize,
}

impl Default for ConnectionInfo {
    fn default() -> Self {
        ConnectionInfo {
            id: String::from(""),
            host: String::from("localhost"),
            port: 1883,
            keep_alive: 5,
            inflight: 10,
            clean_session: false,
            cap: 10,
        }
    }
}

pub type TopicInfo<S> = (S, QoS);

pub async fn new_connection<S: Into<String> + Copy>(
    connection_info: ConnectionInfo,
    subscriptions: &[TopicInfo<S>],
) -> Result<(AsyncClient, EventLoop), Box<dyn Error>> {
    let mut mqttoptions = MqttOptions::new(
        connection_info.id.clone(),
        connection_info.host.clone(),
        connection_info.port,
    );
    mqttoptions
        .set_keep_alive(connection_info.keep_alive)
        .set_inflight(connection_info.inflight)
        .set_clean_session(connection_info.clean_session);

    let (client, eventloop) = AsyncClient::new(mqttoptions, connection_info.cap);

    for &(topic, qos) in subscriptions.iter() {
        client.subscribe(topic, qos).await?;
    }

    Ok((client, eventloop))
}

async fn subscription_loop(
    tx: mpsc::Sender<ConnectionMessage>,
    mut eventloop: EventLoop,
) -> Result<(), Box<dyn Error>> {
    loop {
        match eventloop.poll().await {
            Result::Ok(Event::Incoming(Packet::Publish(publish))) => {
                tx.send(ConnectionMessage::from(publish)).await?
            }
            Result::Ok(event) => {
                log::debug!("Ignored -> {:?}", event);
            }
            Result::Err(ConnectionError::Cancel) => {
                break;
            }
            Result::Err(error) => {
                tx.send(ConnectionMessage {
                    topic: "SYSMR/control/exit".into(),
                    payload: error.to_string().into(),
                    ..Default::default()
                })
                .await?;
                Result::Err(error)?
            }
        }
    }

    Result::Ok(())
}

pub fn task_subscription_loop(
    tx: &mpsc::Sender<ConnectionMessage>,
    eventloop: EventLoop,
) -> task::JoinHandle<()> {
    let subs_tx = tx.clone();
    task::spawn(async move {
        match subscription_loop(subs_tx, eventloop).await {
            Result::Ok(_) => {}
            Result::Err(error) => {
                log::warn!("MQTT error {}", error);
            }
        }
        log::info!("Exiting spawn mqtt subscription...");
    })
}

async fn publication_loop(
    mut rx: broadcast::Receiver<ConnectionResult>,
    client: AsyncClient,
) -> Result<(), Box<dyn Error>> {
    // This is the future in charge of publishing result messages and canceling if final
    while let Ok(res) = rx.recv().await {
        for elem in res.messages.into_iter() {
            client
                .publish(
                    elem.topic,
                    elem.qos,
                    elem.retain,
                    Vec::from(&elem.payload[..]),
                )
                .await?;
        }

        if res.is_final {
            client.cancel().await?;
        }
    }

    Result::Ok(())
}

pub fn task_publication_loop(
    rx: broadcast::Receiver<ConnectionResult>,
    client: AsyncClient,
) -> task::JoinHandle<()> {
    task::spawn(async move {
        match publication_loop(rx, client).await {
            Result::Ok(_) => {}
            Result::Err(error) => {
                log::warn!("MQTT error {}", error);
            }
        }
        log::info!("Exiting spawn mqtt publication...");
    })
}
