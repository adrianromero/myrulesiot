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

use std::collections::HashMap;

use myrulesiot::mqtt::{
    self, EngineAction, EngineFunction, EngineResult, EngineState, ReducerFunction,
};
use myrulesiot::rules::forward;
use myrulesiot::runtime;
use serde_json::json;
use tokio::sync::mpsc;

#[tokio::test]
async fn internal() {
    let (sub_tx, sub_rx) = mpsc::channel::<EngineAction>(10);
    let (pub_tx, mut pub_rx) = mpsc::channel::<EngineResult>(10);

    // Push function
    sub_tx
        .send(EngineAction {
            topic: "MYRULESTEST/command/functions_push".into(),
            payload: b"{\"name\":\"forward_user_action\", \"parameters\": {\"topic\":\"SYSTIMER/tick\",\"forwardtopic\":\"myhelloiot/timer\"}}".into(),
            timestamp: 0,
        })
        .await
        .unwrap();

    // Forward user action tick
    sub_tx
        .send(EngineAction {
            topic: "source_topic".into(),
            payload: b"{\"action\":\"toggle\"}".into(),
            timestamp: 0,
        })
        .await
        .unwrap();
    sub_tx
        .send(EngineAction {
            topic: "SYSTIMER/tick".into(),
            payload: b"123".into(),
            timestamp: 0,
        })
        .await
        .unwrap();
    sub_tx
        .send(EngineAction {
            topic: "MYRULESTEST/command/exit".into(),
            payload: vec![],
            timestamp: 0,
        })
        .await
        .unwrap();

    let engine_functions: HashMap<String, EngineFunction> = HashMap::from([
        (
            String::from("forward_action"),
            forward::engine_forward_action as EngineFunction,
        ),
        (
            String::from("forward_user_action"),
            forward::engine_forward_user_action as EngineFunction,
        ),
    ]);

    let init_state = EngineState::new(
        Default::default(),
        vec![ReducerFunction::new(
            "forward_action".into(),
            json!({ "topic":"source_topic", "forwardtopic":"target_topic"}),
        )],
    );

    runtime::task_runtime_loop(
        &pub_tx,
        sub_rx,
        mqtt::MasterEngine::new(String::from("MYRULESTEST"), engine_functions),
        init_state,
    )
    .await
    .unwrap();

    std::mem::drop(sub_tx);
    std::mem::drop(pub_tx);

    // The function push result
    let result = pub_rx.recv().await.unwrap();
    assert_eq!(
        "EngineResult { messages: [], is_final: false }",
        format!("{:?}", result)
    );

    // The forward action result.
    let result = pub_rx.recv().await.unwrap();
    assert_eq!(
        "EngineResult { messages: [EngineMessage { qos: AtMostOnce, retain: false, topic: \"target_topic\", payload: [1] }], is_final: false }",
        format!("{:?}", result)
    );

    // The forward user action tick result.
    let result = pub_rx.recv().await.unwrap();
    assert_eq!(
        "EngineResult { messages: [EngineMessage { qos: AtMostOnce, retain: false, topic: \"myhelloiot/timer\", payload: [49, 50, 51] }], is_final: false }",
        format!("{:?}", result)
    );

    let result = pub_rx.recv().await.unwrap();
    assert_eq!(
        "EngineResult { messages: [], is_final: true }",
        format!("{:?}", result)
    );

    let result = pub_rx.recv().await;
    assert!(result.is_none());
}
