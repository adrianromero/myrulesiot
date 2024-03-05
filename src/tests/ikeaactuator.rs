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

use super::runtimetester::RuntimeTester;
use crate::mqtt::EngineAction;

#[tokio::test]
async fn basic_messages() {
    let mut testengine = RuntimeTester::new();

    // Push function
    testengine.send(EngineAction {
        topic: "MYRULESTEST/command/functions_push".into(),
        payload: b"{\"name\":\"ikea_actuator\", \"parameters\": {\"topic\":\"zigbee2mqtt/Tradfri Remote\",\"command\":\"toggle\"}}".into(),
        timestamp: 0,
    })
    .await;

    // Push function
    testengine.send(EngineAction {
        topic: "MYRULESTEST/command/functions_push".into(),
        payload: b"{\"name\":\"shelly_relay\", \"parameters\": {\"topic\":\"shellies/shellyswitch01/relay/1/command\"}}".into(),
        timestamp: 0,
    })
    .await;

    testengine
        .send(EngineAction {
            topic: "zigbee2mqtt/Tradfri Remote".into(),
            payload: b"{\"action\":\"toggle\"}".into(),
            timestamp: 0,
        })
        .await;

    testengine
        .send(EngineAction {
            topic: "MYRULESTEST/command/exit".into(),
            payload: vec![],
            timestamp: 0,
        })
        .await;

    testengine.runtime_loop().await;

    // The function push result
    assert_eq!(
        "EngineResult { messages: [], is_final: false }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    // The function push result
    assert_eq!(
        "EngineResult { messages: [], is_final: false }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    // The actuator action result.
    assert_eq!(
        "EngineResult { messages: [EngineMessage { qos: AtMostOnce, retain: false, topic: \"shellies/shellyswitch01/relay/1/command\", payload: [111, 110] }], is_final: false }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    assert_eq!(
        "EngineResult { messages: [], is_final: true }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    assert!(testengine.recv().await.is_none());
}
