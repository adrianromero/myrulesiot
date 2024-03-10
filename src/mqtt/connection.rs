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

use rumqttc::{
    self, AsyncClient, ClientError, ConnectionError, Event, EventLoop, MqttOptions, Packet,
    Publish, QoS,
};
use tokio::sync::mpsc;
use tokio::task;

use super::{EngineAction, EngineResult};

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

pub type TopicInfo = (String, i32);

fn to_engineaction(p: Publish) -> EngineAction {
    EngineAction::new(p.topic, p.payload.into())
}

pub async fn new_connection(
    connection_info: ConnectionValues,
    subscriptions: Vec<TopicInfo>,
) -> Result<(AsyncClient, EventLoop), ClientError> {
    log::info!("MQTT {:?}", &connection_info);
    log::info!("MQTT Subscriptions {:?}", &subscriptions);

    let mut mqttoptions = MqttOptions::new(
        connection_info.id,
        connection_info.host,
        connection_info.port,
    );

    mqttoptions
        .set_keep_alive(connection_info.keep_alive)
        .set_inflight(connection_info.inflight)
        .set_clean_session(connection_info.clean_session);

    let (client, eventloop) = AsyncClient::new(mqttoptions, connection_info.cap);

    for (topic, qos) in subscriptions.into_iter() {
        client
            .subscribe(topic, to_qos(qos).unwrap_or(QoS::AtLeastOnce))
            .await?;
    }

    Ok((client, eventloop))
}

pub fn task_subscription_loop(
    tx: &mpsc::Sender<EngineAction>,
    mut eventloop: EventLoop,
) -> task::JoinHandle<()> {
    let subs_tx = tx.clone();
    task::spawn(async move {
        log::debug!("Started MQTT subscription...");
        loop {
            let event = eventloop.poll().await;
            log::debug!("EventLoop Event -> {:?}", event);
            match event {
                Result::Ok(Event::Incoming(Packet::Publish(publish))) => {
                    if let Err(error) = subs_tx.send(to_engineaction(publish)).await {
                        log::warn!("Exited MQTT subscription with error {}", error);
                        return;
                    }
                }
                Result::Ok(_) => {}
                Result::Err(ConnectionError::Cancel) => {
                    log::debug!("Exited MQTT subscription...");
                    return;
                }
                Result::Err(error) => {
                    log::warn!("Exited MQTT subscription with error {}", error);
                    return;
                }
            }
        }
    })
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

pub fn task_publication_loop(
    mut rx: mpsc::Receiver<EngineResult>,
    client: AsyncClient,
) -> task::JoinHandle<()> {
    task::spawn(async move {
        log::debug!("Started MQTT publication...");
        while let Some(res) = rx.recv().await {
            for elem in res.messages {
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
                    log::warn!("Exited MQTT publication with error {}", error);
                    return;
                }
            }

            if res.is_final {
                if let Err(error) = client.cancel().await {
                    log::warn!("Exited MQTT publication with error {}", error);
                    return;
                }
            }
        }
        log::debug!("Exited MQTT publication...");
    })
}
