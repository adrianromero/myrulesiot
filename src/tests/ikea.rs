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

use serde_json::{json, Value};

use super::runtimetester::RuntimeTester;
use crate::master::{EngineAction, EngineStatus, FinalStatus};

#[tokio::test]
async fn basic_messages() {
    let mut testengine = RuntimeTester::new();

    // Push function
    testengine
        .send(EngineAction::new(
            String::from("MYRULESTEST/command/functions_push"),
            Vec::from( 
                b"{\"name\":\"start_ikea_remote_toggle\", \"_topic\":\"zigbee2mqtt/Tradfri Remote\"}"
            ),
        ))
        .await;

    testengine
        .send(EngineAction::new(
            String::from("MYRULESTEST/command/functions_push"),
            Vec::from(
                b"{\"name\":\"relay_on\", \"_topic\":\"shellies/shellyswitch01/relay/1/command\"}",
            ),
        ))
        .await;

    testengine
        .send(EngineAction::new(
            String::from("zigbee2mqtt/Tradfri Remote"),
            Vec::from(b"{\"action\":\"toggle\"}"),
        ))
        .await;

    testengine
        .send(EngineAction::new(String::from("MYRULESTEST/command/exit"), Vec::new()))
        .await;

    let state = testengine.runtime_loop().await;

    // The function push result
    let t = testengine.recv().await.unwrap();
    assert_eq!(t.messages.len(), 1);
    assert_eq!(&t.messages[0].topic, "MYRULESTEST/notify/functions_push");
    assert_eq!(
        json!({
            "function" : "start_ikea_remote_toggle",
            "success" : true
        }),
        serde_json::from_slice::<Value>(&t.messages[0].payload).unwrap()
    );

    // The function push result  
    let t = testengine.recv().await.unwrap();
    assert_eq!(t.messages.len(), 1);
    assert_eq!(&t.messages[0].topic, "MYRULESTEST/notify/functions_push");
    assert_eq!(
        json!({
            "function" : "relay_on",
            "success" : true
        }),
        serde_json::from_slice::<Value>(&t.messages[0].payload).unwrap()
    );
 
    // The actuator action result.
    assert_eq!(
        "EngineResult { messages: [EngineMessage { topic: \"shellies/shellyswitch01/relay/1/command\", payload: [111, 110], properties: Null }] }",
        format!("{:?}", testengine.recv().await.unwrap())
    );

    assert!(matches!(
        state.engine_status,
        EngineStatus::FINAL(FinalStatus::NORMAL, _)
    ));
    assert_eq!(
        json!([{
            "_topic": "zigbee2mqtt/Tradfri Remote",
            "name": "start_ikea_remote_toggle"
        }, {
            "_topic": "shellies/shellyswitch01/relay/1/command",
            "name":"relay_on"
        }]),
        serde_json::to_value(&state.functions).unwrap()
    );

    let final_result = testengine.recv().await.unwrap();
    assert_eq!(final_result.messages.len(), 0);

    assert!(testengine.recv().await.is_none());
}
