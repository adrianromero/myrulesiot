use myrulesiot::mqtt::{self, ConnectionAction, ConnectionResult};
use myrulesiot::rules::forward;
use myrulesiot::runtime;
use tokio::sync::mpsc;

#[tokio::test]
async fn internal() {
    let (sub_tx, sub_rx) = mpsc::channel::<ConnectionAction>(10);
    let (pub_tx, mut pub_rx) = mpsc::channel::<ConnectionResult>(10);

    let reducers: Vec<mqtt::FnMQTTReducer> = vec![Box::new(forward::forward_user_action_tick(
        "myhelloiot/timer",
    ))];

    sub_tx
        .send(ConnectionAction {
            topic: "SYSMR/user_action/tick".to_string(),
            payload: b"aaa".into(),
            timestamp: 0,
        })
        .await
        .unwrap();
    sub_tx
        .send(ConnectionAction {
            topic: "SYSMR/system_action".into(),
            payload: b"exit".into(),
            timestamp: 0,
        })
        .await
        .unwrap();

    runtime::task_runtime_loop(
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
        "ConnectionResult { messages: [ConnectionMessage { qos: AtMostOnce, retain: false, topic: \"myhelloiot/timer\", payload: [97, 97, 97] }], is_final: false }",
        format!("{:?}", result)
    );

    let result = pub_rx.recv().await.unwrap();
    assert_eq!(
        "ConnectionResult { messages: [], is_final: true }",
        format!("{:?}", result)
    );

    let result = pub_rx.recv().await;
    assert!(result.is_none());
}
