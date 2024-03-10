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
    testengine.send(EngineAction::new(
        "MYRULESTEST/command/functions_push".into(),
        b"{\"name\":\"ikea_actuator\", \"parameters\": {\"topic\":\"zigbee2mqtt/Tradfri Remote\",\"command\":\"toggle\"}}".into()
    ))
    .await;

    testengine
    .send(EngineAction::new(
        "MYRULESTEST/command/functions_push".into(),
        b"{\"name\":\"shelly_relay\", \"parameters\": {\"topic\":\"shellies/shellyswitch01/relay/1/command\"}}".into(),
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
    assert_eq!(
        "EngineResult { messages: [EngineMessage { topic: \"MYRULESTEST/notify/functions_push\", payload: [123, 34, 102, 117, 110, 99, 116, 105, 111, 110, 34, 58, 34, 105, 107, 101, 97, 95, 97, 99, 116, 117, 97, 116, 111, 114, 34, 44, 34, 115, 117, 99, 99, 101, 115, 115, 34, 58, 116, 114, 117, 101, 125], properties: Null }], is_final: false }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    // The function push result
    assert_eq!(
        "EngineResult { messages: [EngineMessage { topic: \"MYRULESTEST/notify/functions_push\", payload: [123, 34, 102, 117, 110, 99, 116, 105, 111, 110, 34, 58, 34, 115, 104, 101, 108, 108, 121, 95, 114, 101, 108, 97, 121, 34, 44, 34, 115, 117, 99, 99, 101, 115, 115, 34, 58, 116, 114, 117, 101, 125], properties: Null }], is_final: false }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    // The actuator action result.
    assert_eq!(
        "EngineResult { messages: [EngineMessage { topic: \"shellies/shellyswitch01/relay/1/command\", payload: [111, 110], properties: Null }], is_final: false }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    assert_eq!(
        "EngineResult { messages: [], is_final: true }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    assert!(testengine.recv().await.is_none());
}
