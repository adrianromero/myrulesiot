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

use std::fmt;

use rumqttc::{
    self, AsyncClient, ClientError, ConnectionError, Event, EventLoop, MqttOptions, Packet, QoS,
};
use std::error::Error;
use tokio::sync::mpsc;
use tokio::task;

use super::{ActionMessage, ConnectionResult};

#[derive(Debug)]
pub struct ConnectionValues {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub keep_alive: u16,
    pub inflight: u16,
    pub clean_session: bool,
    pub cap: usize,
}

impl Default for ConnectionValues {
    fn default() -> Self {
        ConnectionValues {
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

pub type TopicInfo<'a> = (&'a str, QoS);

pub async fn new_connection<'a>(
    connection_info: ConnectionValues,
    subscriptions: &[TopicInfo<'a>],
) -> Result<(AsyncClient, EventLoop), ClientError> {
    log::info!("MQTT {:?}", &connection_info);
    log::info!("MQTT Subscriptions {:?}", &subscriptions);

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

    for &(topic, qos) in subscriptions {
        client.subscribe(topic, qos).await?;
    }

    Ok((client, eventloop))
}

async fn subscription_loop(
    tx: mpsc::Sender<ActionMessage>,
    mut eventloop: EventLoop,
) -> Result<(), Box<dyn Error>> {
    loop {
        let event = eventloop.poll().await;
        log::debug!("EventLoop Event -> {:?}", event);
        match event {
            Result::Ok(Event::Incoming(Packet::Publish(publish))) => {
                tx.send(ActionMessage::from(publish)).await?;
            }
            Result::Ok(_) => {}
            Result::Err(ConnectionError::Cancel) => {
                break Result::Ok(());
            }
            Result::Err(error) => return Result::Err(Box::new(error)),
        }
    }
}

pub fn task_subscription_loop(
    tx: &mpsc::Sender<ActionMessage>,
    eventloop: EventLoop,
) -> task::JoinHandle<()> {
    let subs_tx = tx.clone();
    task::spawn(async move {
        log::debug!("Started MQTT subscription...");
        match subscription_loop(subs_tx, eventloop).await {
            Result::Ok(_) => {}
            Result::Err(error) => {
                log::warn!("Subscription error {:?}", error);
            }
        }
        log::debug!("Exited MQTT subscription...");
    })
}

async fn publication_loop(
    mut rx: mpsc::Receiver<ConnectionResult>,
    client: AsyncClient,
) -> Result<(), rumqttc::ClientError> {
    // This is the future in charge of publishing result messages and canceling if final
    while let Some(res) = rx.recv().await {
        for elem in res.messages.into_iter() {
            log::debug!("Publication loop -> {:?}", elem);
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
    rx: mpsc::Receiver<ConnectionResult>,
    client: AsyncClient,
) -> task::JoinHandle<()> {
    task::spawn(async move {
        log::debug!("Started MQTT publication...");
        match publication_loop(rx, client).await {
            Result::Ok(_) => {}
            Result::Err(error) => {
                log::warn!("Publication error {}", error);
            }
        }
        log::debug!("Started MQTT publication...");
    })
}
