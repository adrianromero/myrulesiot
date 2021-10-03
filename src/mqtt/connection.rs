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

use rumqttc::{
    self, AsyncClient, ConnectionError, Event, EventLoop, MqttOptions, Packet, Publish, QoS,
};
use std::error::Error;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct ConnectionInfo {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub keep_alive: u16,
    pub inflight: u16,
    pub clean_session: bool,
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
        }
    }
}

pub type TopicInfo = (String, QoS);

pub struct Connection;
impl Connection {
    pub async fn new(
        connection_info: ConnectionInfo,
        subscriptions: Vec<TopicInfo>,
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

        let (client, eventloop) = AsyncClient::new(mqttoptions, 10);

        for (topic, qos) in subscriptions.into_iter() {
            client.subscribe(topic, qos).await?;
        }

        Ok((client, eventloop))
    }

    pub async fn do_loop(
        mut eventloop: EventLoop,
        tx: mpsc::Sender<Publish>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            let result = eventloop.poll().await;
            match result {
                Result::Ok(Event::Incoming(Packet::Publish(publish))) => {
                    tx.send(publish).await?;
                }
                Result::Ok(event) => {
                    log::debug!("Ignored -> {:?}", event);
                }
                Result::Err(ConnectionError::Cancel) => {
                    break;
                }
                Result::Err(error) => {
                    log::warn!("Error -> {:?}", error);
                    Result::Err(error)?;
                }
            }
        }

        Result::Ok(())
    }
}
