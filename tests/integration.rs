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

use myrulesiot::mqtt::{self, EngineAction, EngineResult};
use myrulesiot::rules::forward;
use myrulesiot::runtime;
use tokio::sync::mpsc;

#[tokio::test]
async fn internal() {
    let (sub_tx, sub_rx) = mpsc::channel::<EngineAction>(10);
    let (pub_tx, mut pub_rx) = mpsc::channel::<EngineResult>(10);

    let reducers: Vec<mqtt::FnMQTTReducer> = vec![
        Box::new(forward::box_forward_action("source_topic", "target_topic")),
        Box::new(forward::box_forward_user_action_tick("myhelloiot/timer")),
    ];

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
            topic: "SYSMR/user_action/tick".into(),
            payload: b"aaa".into(),
            timestamp: 0,
        })
        .await
        .unwrap();
    sub_tx
        .send(EngineAction {
            topic: "SYSMR/system_action".into(),
            payload: b"exit".into(),
            timestamp: 0,
        })
        .await
        .unwrap();

    runtime::task_runtime_init_loop(
        &pub_tx,
        sub_rx,
        mqtt::ConnectionEngine::new(mqtt::create_reducer(reducers)),
    )
    .await
    .unwrap();

    std::mem::drop(sub_tx);
    std::mem::drop(pub_tx);

    let result = pub_rx.recv().await.unwrap();
    assert_eq!(
        "EngineResult { messages: [EngineMessage { qos: AtMostOnce, retain: false, topic: \"target_topic\", payload: [1] }], is_final: false }",
        format!("{:?}", result)
    );

    let result = pub_rx.recv().await.unwrap();
    assert_eq!(
        "EngineResult { messages: [EngineMessage { qos: AtMostOnce, retain: false, topic: \"myhelloiot/timer\", payload: [97, 97, 97] }], is_final: false }",
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
