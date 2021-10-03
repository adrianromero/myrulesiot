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

use rumqttc::QoS;

use crate::engine::Engine;
use crate::mqtt::{self, ConnectionMessage, ConnectionResult, ConnectionState};

pub fn create_main_engine() -> Engine<ConnectionMessage, ConnectionResult, ConnectionState<u32>> {
    mqtt::create_engine(|state: &ConnectionState<u32>, action: &ConnectionMessage| {
        let mut messages = Vec::<ConnectionMessage>::new();
        if "myhelloiot/alarm".eq(&action.topic) {
            messages.push(ConnectionMessage {
                topic: "myhelloiot/modal".into(),
                qos: QoS::AtMostOnce,
                retain: false,
                payload: "0".into(),
            })
        }

        let actionfinal = "myhelloiot/exit".eq(&action.topic) && "1234".eq(&action.payload);

        //if action.message
        ConnectionState {
            info: state.info + 1,
            messages,
            is_final: state.info == 120 || actionfinal,
        }
    })
}
