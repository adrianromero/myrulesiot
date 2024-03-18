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
        b"{\"name\":\"actuator_ikea_remote_toggle\", \"parameters\": {\"topic\":\"zigbee2mqtt/Tradfri Remote\"}}".into()
    ))
    .await;

    testengine
    .send(EngineAction::new(
        "MYRULESTEST/command/functions_push".into(),
        b"{\"name\":\"relay_on\", \"parameters\": {\"topic\":\"shellies/shellyswitch01/relay/1/command\"}}".into(),
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
        "EngineResult { messages: [EngineMessage { topic: \"MYRULESTEST/notify/functions_push\", payload: [123, 34, 102, 117, 110, 99, 116, 105, 111, 110, 34, 58, 34, 97, 99, 116, 117, 97, 116, 111, 114, 95, 105, 107, 101, 97, 95, 114, 101, 109, 111, 116, 101, 95, 116, 111, 103, 103, 108, 101, 34, 44, 34, 115, 117, 99, 99, 101, 115, 115, 34, 58, 116, 114, 117, 101, 125], properties: Null }], is_final: false }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

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

    assert_eq!(
        "EngineResult { messages: [EngineMessage { topic: \"MYRULESTEST/notify/exit_functions\", payload: [91, 123, 34, 110, 97, 109, 101, 34, 58, 34, 97, 99, 116, 117, 97, 116, 111, 114, 95, 105, 107, 101, 97, 95, 114, 101, 109, 111, 116, 101, 95, 116, 111, 103, 103, 108, 101, 34, 44, 34, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 34, 58, 123, 34, 116, 111, 112, 105, 99, 34, 58, 34, 122, 105, 103, 98, 101, 101, 50, 109, 113, 116, 116, 47, 84, 114, 97, 100, 102, 114, 105, 32, 82, 101, 109, 111, 116, 101, 34, 125, 125, 44, 123, 34, 110, 97, 109, 101, 34, 58, 34, 114, 101, 108, 97, 121, 95, 111, 110, 34, 44, 34, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 34, 58, 123, 34, 116, 111, 112, 105, 99, 34, 58, 34, 115, 104, 101, 108, 108, 105, 101, 115, 47, 115, 104, 101, 108, 108, 121, 115, 119, 105, 116, 99, 104, 48, 49, 47, 114, 101, 108, 97, 121, 47, 49, 47, 99, 111, 109, 109, 97, 110, 100, 34, 125, 125, 93], properties: Null }, EngineMessage { topic: \"MYRULESTEST/notify/exit\", payload: [123, 34, 115, 117, 99, 99, 101, 115, 115, 34, 58, 116, 114, 117, 101, 125], properties: Null }], is_final: true }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    assert!(testengine.recv().await.is_none());
}
