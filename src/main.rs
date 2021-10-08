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

use rumqttc::{self, QoS};
use std::error::Error;

mod mqtt;
use mqtt::{ConnectionInfo, ConnectionMessage, ConnectionState};
mod engine;
mod timer;

#[derive(Debug)]
pub struct AppInfo {
    one: String,
    two: i32,
    three: Vec<String>,
}

impl Default for AppInfo {
    fn default() -> Self {
        AppInfo {
            one: "".into(),
            two: 0,
            three: vec![],
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let engine = mqtt::create_engine(
        |state: ConnectionState<AppInfo>, action: ConnectionMessage| {
            if "$MYRULESIOTSYSTEM/timer".eq(&action.topic) {
                return ConnectionState {
                    info: state.info,
                    messages: vec![ConnectionMessage {
                        topic: "myhelloiot/timer".into(),
                        qos: QoS::AtMostOnce,
                        retain: false,
                        payload: action.payload,
                    }],
                    is_final: false,
                };
            }

            let mut messages = Vec::<ConnectionMessage>::new();
            if "myhelloiot/alarm".eq(&action.topic) {
                messages.push(ConnectionMessage {
                    topic: "myhelloiot/modal".into(),
                    qos: QoS::AtMostOnce,
                    retain: false,
                    payload: "0".into(),
                })
            }

            let is_final = "myhelloiot/exit".eq(&action.topic) && "1234".eq(&action.payload);

            //if action.message
            ConnectionState {
                info: AppInfo {
                    two: state.info.two + 1,
                    ..Default::default()
                },
                messages,
                is_final,
            }
        },
    );

    // Defines connection properties
    let connection_info = ConnectionInfo {
        id: "rustclient-231483".into(),
        host: "localhost".into(),
        clean_session: true,
        ..Default::default()
    };
    let subscriptions = &[("myhelloiot/#", QoS::AtMostOnce)];

    // This goes to mqtt module
    mqtt::connection_engine(engine, connection_info, subscriptions).await
}
