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

use rumqttc::{self, AsyncClient, EventLoop, MqttOptions, QoS};

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

pub struct Connection {
    connection_info: ConnectionInfo,
    async_client: Option<(AsyncClient, EventLoop)>,
}

impl Connection {
    fn new(connection_info: ConnectionInfo) -> Self {
        Connection {
            connection_info: connection_info,
            async_client: None,
        }
    }

    fn connect(&mut self) {
        let mut mqttoptions = MqttOptions::new(
            self.connection_info.id.clone(),
            self.connection_info.host.clone(),
            self.connection_info.port,
        );
        mqttoptions
            .set_keep_alive(self.connection_info.keep_alive)
            .set_inflight(self.connection_info.inflight)
            .set_clean_session(self.connection_info.clean_session);

        let (client, eventloop) = AsyncClient::new(mqttoptions, 10);
        self.async_client = Some((client, eventloop));
    }
}
