//    MyRulesIoT  Project is a rules engine for MQTT based on MyRulesIoT lib
//    Copyright (C) 2024 Adrián Romero Corchado.
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

use crate::master::EngineAction;
use crate::rules::savelist::save_list;
use serde_json::json;

#[test]
fn empty_savelist() {
    let mut info = json!({
        "_topic": "savelist_topic",
        "_value": 1000,
        "_count": 5,
        "_timestamp": 1000
    });

    let result = save_list(
        &info,
        &EngineAction::new_json("savelist_topic".into(), json!(100)),
    );

    assert_eq!(
        json!({"savelist_topic/list":{"current": 100}}),
        result.state
    );

    // Step 2
    json_patch::merge(&mut info, &result.state);
    info["_timestamp"] = json!(2000);

    let result = save_list(
        &info,
        &EngineAction::new_json("SYSMR/action/tick".into(), json!(null)),
    );

    assert_eq!(
        json!({"savelist_topic/list":{
            "valuest": 2000,
            "values": [null, null, null, null, 100]
        }}),
        result.state
    );

    // Step 3
    json_patch::merge(&mut info, &result.state);
    info["_timestamp"] = json!(2200);

    let result = save_list(
        &info,
        &EngineAction::new_json("SYSMR/action/tick".into(), json!(null)),
    );

    assert_eq!(
        json!({"savelist_topic/list":{
            "valuest": 2200,
            "values": [null, null, null, 100, 100]
        }}),
        result.state
    );
}
