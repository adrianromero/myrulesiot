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
        payload: b"{\"name\":\"forward_action\", \"parameters\": {\"topic\":\"source_topic\",\"forwardtopic\":\"target_topic\"}}".into(),
        timestamp: 0,
    })
    .await;

    // Push function
    testengine.send(EngineAction {
        topic: "MYRULESTEST/command/functions_push".into(),
        payload: b"{\"name\":\"forward_user_action\", \"parameters\": {\"topic\":\"SYSTIMER/tick\",\"forwardtopic\":\"myhelloiot/timer\"}}".into(),
        timestamp: 0,
    })
    .await;

    // Forward user action tick
    testengine
        .send(EngineAction {
            topic: "source_topic".into(),
            payload: b"{\"action\":\"toggle\"}".into(),
            timestamp: 0,
        })
        .await;
    testengine
        .send(EngineAction {
            topic: "SYSTIMER/tick".into(),
            payload: b"123".into(),
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
        "EngineResult { messages: [EngineMessage { topic: \"MYRULESTEST/notify/functions_push\", payload: [123, 34, 102, 117, 110, 99, 116, 105, 111, 110, 34, 58, 34, 102, 111, 114, 119, 97, 114, 100, 95, 97, 99, 116, 105, 111, 110, 34, 44, 34, 115, 117, 99, 99, 101, 115, 115, 34, 58, 116, 114, 117, 101, 125], properties: Null }], is_final: false }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    // The function push result
    assert_eq!(
        "EngineResult { messages: [EngineMessage { topic: \"MYRULESTEST/notify/functions_push\", payload: [123, 34, 102, 117, 110, 99, 116, 105, 111, 110, 34, 58, 34, 102, 111, 114, 119, 97, 114, 100, 95, 117, 115, 101, 114, 95, 97, 99, 116, 105, 111, 110, 34, 44, 34, 115, 117, 99, 99, 101, 115, 115, 34, 58, 116, 114, 117, 101, 125], properties: Null }], is_final: false }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    // The forward action result.
    assert_eq!(
        "EngineResult { messages: [EngineMessage { topic: \"target_topic\", payload: [1], properties: Null }], is_final: false }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    // The forward user action tick result.
    assert_eq!(
        "EngineResult { messages: [EngineMessage { topic: \"myhelloiot/timer\", payload: [49, 50, 51], properties: Null }], is_final: false }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    assert_eq!(
        "EngineResult { messages: [EngineMessage { topic: \"MYRULESTEST/notify/exit_functions\", payload: [91, 123, 34, 110, 97, 109, 101, 34, 58, 34, 102, 111, 114, 119, 97, 114, 100, 95, 97, 99, 116, 105, 111, 110, 34, 44, 34, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 34, 58, 123, 34, 102, 111, 114, 119, 97, 114, 100, 116, 111, 112, 105, 99, 34, 58, 34, 116, 97, 114, 103, 101, 116, 95, 116, 111, 112, 105, 99, 34, 44, 34, 116, 111, 112, 105, 99, 34, 58, 34, 115, 111, 117, 114, 99, 101, 95, 116, 111, 112, 105, 99, 34, 125, 125, 44, 123, 34, 110, 97, 109, 101, 34, 58, 34, 102, 111, 114, 119, 97, 114, 100, 95, 117, 115, 101, 114, 95, 97, 99, 116, 105, 111, 110, 34, 44, 34, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 34, 58, 123, 34, 102, 111, 114, 119, 97, 114, 100, 116, 111, 112, 105, 99, 34, 58, 34, 109, 121, 104, 101, 108, 108, 111, 105, 111, 116, 47, 116, 105, 109, 101, 114, 34, 44, 34, 116, 111, 112, 105, 99, 34, 58, 34, 83, 89, 83, 84, 73, 77, 69, 82, 47, 116, 105, 99, 107, 34, 125, 125, 93], properties: Null }, EngineMessage { topic: \"MYRULESTEST/notify/exit\", payload: [123, 34, 115, 117, 99, 99, 101, 115, 115, 34, 58, 116, 114, 117, 101, 125], properties: Null }], is_final: true }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    assert!(testengine.recv().await.is_none());
}
