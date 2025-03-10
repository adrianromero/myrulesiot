//    MyRulesIoT  Project is a rules engine for MQTT based on MyRulesIoT lib
//    Copyright (C) 2022-2025 Adrián Romero Corchado.
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

use serde_json::json;

use super::runtimetester::RuntimeTester;
use crate::master::{EngineAction, EngineStatus, FinalStatus};

#[tokio::test]
async fn basic_messages() {
    let mut testengine = RuntimeTester::new();

    // Push function
    testengine.send(EngineAction::new("MYRULESTEST/command/functions_push".into(),
         b"{\"name\":\"forward_action\", \"_topic\":\"source_topic\",\"_forwardtopic\":\"target_topic\"}".into()))
    .await;

    // Push function
    testengine.send(EngineAction::new("MYRULESTEST/command/functions_push".into(),
        b"{\"name\":\"forward_user_action\", \"_topic\":\"SYSTIMER/tick\",\"_forwardtopic\":\"myhelloiot/timer\"}".into(),
))
    .await;

    // Forward user action tick
    testengine
        .send(EngineAction::new(
            "source_topic".into(),
            b"{\"action\":\"toggle\"}".into(),
        ))
        .await;
    testengine
        .send(EngineAction::new("SYSTIMER/tick".into(), b"123".into()))
        .await;
    testengine
        .send(EngineAction::new("MYRULESTEST/command/exit".into(), vec![]))
        .await;

    let state = testengine.runtime_loop().await;

    // The function push result
    assert_eq!(
        "EngineResult { messages: [EngineMessage { topic: \"MYRULESTEST/notify/functions_push\", payload: [123, 34, 102, 117, 110, 99, 116, 105, 111, 110, 34, 58, 34, 102, 111, 114, 119, 97, 114, 100, 95, 97, 99, 116, 105, 111, 110, 34, 44, 34, 115, 117, 99, 99, 101, 115, 115, 34, 58, 116, 114, 117, 101, 125], properties: Null }] }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    // The function push result
    assert_eq!(
        "EngineResult { messages: [EngineMessage { topic: \"MYRULESTEST/notify/functions_push\", payload: [123, 34, 102, 117, 110, 99, 116, 105, 111, 110, 34, 58, 34, 102, 111, 114, 119, 97, 114, 100, 95, 117, 115, 101, 114, 95, 97, 99, 116, 105, 111, 110, 34, 44, 34, 115, 117, 99, 99, 101, 115, 115, 34, 58, 116, 114, 117, 101, 125], properties: Null }] }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    // The forward action result.
    assert_eq!(
        "EngineResult { messages: [EngineMessage { topic: \"target_topic\", payload: [1], properties: Null }] }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    // The forward user action tick result.
    assert_eq!(
        "EngineResult { messages: [EngineMessage { topic: \"myhelloiot/timer\", payload: [49, 50, 51], properties: Null }] }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    assert!(matches!(
        state.engine_status,
        EngineStatus::FINAL(FinalStatus::NORMAL, _)
    ));

    assert_eq!(
        json!([{
            "name" : "forward_action",
            "_forwardtopic" : "target_topic",
            "_topic" : "source_topic"
        },{
            "name" : "forward_user_action",
            "_forwardtopic" : "myhelloiot/timer",
            "_topic" : "SYSTIMER/tick"
        }]),
        serde_json::to_value(&state.functions).unwrap()
    );

    let final_result = testengine.recv().await.unwrap();
    assert_eq!(final_result.messages.len(), 0);

    assert!(testengine.recv().await.is_none());
}
