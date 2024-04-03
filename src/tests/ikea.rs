//    MyRulesIoT  Project is a rules engine for MQTT based on MyRulesIoT lib
//    Copyright (C) 2022-2024my  Adrián Romero Corchado.
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

use serde_json::{json, Value};

use super::runtimetester::RuntimeTester;
use crate::master::EngineAction;

#[tokio::test]
async fn basic_messages() {
    let mut testengine = RuntimeTester::new();

    // Push function
    testengine
        .send(EngineAction::new(
            "MYRULESTEST/command/functions_push".into(),
            b"{\"name\":\"start_ikea_remote_toggle\", \"_topic\":\"zigbee2mqtt/Tradfri Remote\"}"
                .into(),
        ))
        .await;

    testengine
        .send(EngineAction::new(
            "MYRULESTEST/command/functions_push".into(),
            b"{\"name\":\"relay_on\", \"_topic\":\"shellies/shellyswitch01/relay/1/command\"}"
                .into(),
        ))
        .await;

    testengine
        .send(EngineAction::new(
            "zigbee2mqtt/Tradfri Remote".into(),
            b"{\"action\":\"toggle\"}".into(),
        ))
        .await;

    testengine
        .send(EngineAction::new("MYRULESTEST/command/exit".into(), vec![]))
        .await;

    testengine.runtime_loop().await;

    // The function push result
    let t = testengine.recv().await.unwrap();
    assert_eq!(&t.messages[0].topic, "MYRULESTEST/notify/functions_push");
    assert_eq!(
        json!({
            "function" : "start_ikea_remote_toggle",
            "success" : true
        }),
        serde_json::from_slice::<Value>(&t.messages[0].payload).unwrap()
    );
    assert!(!t.is_final);

    // The function push result
    assert_eq!(
        "EngineResult { messages: [EngineMessage { topic: \"MYRULESTEST/notify/functions_push\", payload: [123, 34, 102, 117, 110, 99, 116, 105, 111, 110, 34, 58, 34, 114, 101, 108, 97, 121, 95, 111, 110, 34, 44, 34, 115, 117, 99, 99, 101, 115, 115, 34, 58, 116, 114, 117, 101, 125], properties: Null }], is_final: false }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    // The actuator action result.
    assert_eq!(
        "EngineResult { messages: [EngineMessage { topic: \"shellies/shellyswitch01/relay/1/command\", payload: [111, 110], properties: Null }], is_final: false }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    // The function push result
    let t = testengine.recv().await.unwrap();
    assert_eq!(&t.messages[0].topic, "SYSMR/notify/save_functions");
    assert_eq!(
        json!([{
            "_topic": "zigbee2mqtt/Tradfri Remote",
            "name": "start_ikea_remote_toggle"
        }, {
            "_topic": "shellies/shellyswitch01/relay/1/command",
            "name":"relay_on"
        }]),
        t.messages[0].payload_into_json().unwrap()
    );
    assert!(t.is_final);

    assert!(testengine.recv().await.is_none());
}
