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

use rumqttc::{AsyncClient, ClientError, EventLoop, QoS};

use crate::mqtt;

pub async fn connect_mqtt() -> Result<(AsyncClient, EventLoop), ClientError> {
    // Defines connection properties
    let connection_info = mqtt::ConnectionInfo {
        id: "rustclient-231483".into(),
        host: "localhost".into(),
        clean_session: true,
        ..Default::default()
    };
    let subscriptions = &[
        ("myhelloiot/#", QoS::AtMostOnce),
        ("zigbee2mqtt/0x000b57fffe4fc5ca", QoS::AtMostOnce),
        ("SYSMR/system_action", QoS::AtMostOnce),
        ("ESPURNITA04/#", QoS::AtMostOnce),
    ];

    mqtt::new_connection(connection_info, subscriptions).await
}
