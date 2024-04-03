//    MyRulesIoT is a rules engine for MQTT
//    Copyright (C) 2021-2024 Adrián Romero Corchado.
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

use thiserror::Error;

use rumqttc::{
    self, AsyncClient, ClientError, ConnectionError, Event, EventLoop, MqttOptions, Packet,
    Publish, QoS,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::master::{EngineAction, EngineResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionValues {
    #[serde(default)]
    pub client_id: String,
    pub host: String,
    pub username: String,
    pub password: String,
    #[serde(default = "port_default")]
    pub port: u16,
    #[serde(default = "keep_alive_default")]
    pub keep_alive: u16,
    #[serde(default = "inflight_default")]
    pub inflight: u16,
    #[serde(default = "clean_session_default")]
    pub clean_session: bool,
    #[serde(default = "cap_default")]
    pub cap: usize,
}

fn port_default() -> u16 {
    1883
}
fn keep_alive_default() -> u16 {
    5
}
fn inflight_default() -> u16 {
    10
}
fn clean_session_default() -> bool {
    false
}
fn cap_default() -> usize {
    10
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub topic: String,
    pub qos: i32,
}

fn to_engineaction(p: Publish) -> EngineAction {
    EngineAction::new(p.topic, p.payload.into())
}

#[derive(Error, Debug)]
pub enum MQTTError {
    #[error("ClientError")]
    ClientError(#[from] ClientError),
    #[error("Cannot connect to the MQTT broker")]
    ConnectionError(#[from] ConnectionError),
    #[error("Unexpected MQTT connection package")]
    Unexpected,
}

pub async fn new_connection(
    connection_info: ConnectionValues,
    subscriptions: Vec<Subscription>,
) -> Result<(AsyncClient, EventLoop), MQTTError> {
    let mut mqttoptions = MqttOptions::new(
        connection_info.client_id,
        connection_info.host,
        connection_info.port,
    );

    mqttoptions
        .set_credentials(connection_info.username, connection_info.password)
        .set_keep_alive(connection_info.keep_alive)
        .set_inflight(connection_info.inflight)
        .set_clean_session(connection_info.clean_session);

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, connection_info.cap);

    match eventloop.poll().await {
        Result::Ok(Event::Incoming(Packet::ConnAck(_connack))) => {
            // Connection successful
        }
        Result::Ok(_) => {
            return Err(MQTTError::Unexpected);
        }
        Result::Err(error) => {
            return Err(error.into());
        }
    }

    for Subscription { topic, qos } in subscriptions.into_iter() {
        client
            .subscribe(topic, to_qos(qos).unwrap_or(QoS::AtLeastOnce))
            .await?;
    }

    Ok((client, eventloop))
}

pub async fn task_subscription_loop(subs_tx: mpsc::Sender<EngineAction>, mut eventloop: EventLoop) {
    log::debug!("Starting MQTT subscription...");
    loop {
        let event = eventloop.poll().await;
        log::trace!("EventLoop Event -> {:?}", event);
        match event {
            Result::Ok(Event::Incoming(Packet::Publish(publish))) => {
                if publish.topic.starts_with("SYSMR/") {
                    // Filter SYSMR/ topics
                    return;
                }
                if let Err(error) = subs_tx.send(to_engineaction(publish)).await {
                    log::warn!("Exiting MQTT subscription with publish error {}", error);
                    return;
                }
            }
            Result::Ok(_) => {}
            Result::Err(ConnectionError::Cancel) => {
                log::debug!("Exiting MQTT subscription...");
                return;
            }
            Result::Err(error) => {
                if let Err(senderror) = subs_tx
                    .send(EngineAction::new(
                        "SYSMR/action/error".into(),
                        error.to_string().into_bytes(),
                    ))
                    .await
                {
                    log::warn!("Cannot send exit with error message {}", senderror);
                }
                log::warn!("Exiting MQTT subscription with client error {}", error);
                return;
            }
        }
    }
}

pub fn to_qos(num: i32) -> Option<QoS> {
    match num {
        0 => Some(QoS::AtMostOnce),
        1 => Some(QoS::AtLeastOnce),
        2 => Some(QoS::ExactlyOnce),
        _nonvalid => None,
    }
}

pub fn from_qos(qos: QoS) -> i64 {
    match qos {
        QoS::AtMostOnce => 0,
        QoS::AtLeastOnce => 1,
        QoS::ExactlyOnce => 2,
    }
}

pub async fn task_publication_loop(mut rx: mpsc::Receiver<EngineResult>, client: AsyncClient) {
    log::debug!("Starting MQTT publication...");
    while let Some(res) = rx.recv().await {
        for elem in res.messages {
            if elem.topic.starts_with("SYSMR/") {
                // Filter SYSMR/ topics
                continue;
            }
            if let Err(error) = client
                .publish(
                    elem.topic,
                    elem.properties["qos"]
                        .as_i64()
                        .and_then(|i| to_qos(i as i32))
                        .unwrap_or(QoS::AtLeastOnce),
                    elem.properties["retain"].as_bool().unwrap_or(false),
                    elem.payload,
                )
                .await
            {
                log::warn!("Exiting MQTT publication with error {}", error);
                return;
            }
        }
    }
    if let Err(error) = client.cancel().await {
        log::warn!("Exiting MQTT publication with error {}", error);
    }
    log::debug!("Exiting MQTT publication...");
}
