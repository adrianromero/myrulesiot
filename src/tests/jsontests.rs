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

use serde_json::{json, Value};

#[test]
fn basic_messages() {
    let mut info = json!({
        "key1": "value1",
        "key2": 2,
        "_key3": "value3",
        "_key4": null,
    });
    let expected_info = json!({
        "key1": "value1",
        "key2": 2,
    });

    // Removes all non persitable keys
    if let Value::Object(obj) = &mut info {
        obj.retain(|k: &String, _v: &mut Value| !k.starts_with("_"));
    }

    assert_eq!(info, expected_info);
}

#[test]
fn merge_messages() {
    let mut info = json!({
        "key1": "value1",
        "key2": 2,
        "_key3": "value3",
        "_key4": null,
    });
    let result = json!({
        "key1": "newvalue",
        "_key3":null,
        "key5": true,
    });
    let expected_result = json!({
        "key1": "newvalue",
        "key2": 2,
        "_key4": null,
        "key5": true,
    });

    json_patch::merge(&mut info, &result);

    assert_eq!(info, expected_result);
}

#[test]
fn serialize_vec() {
    //Vec<Option<Vec<u8>>>,
    let info = json!({
        "key1": vec![1, 2, 4, 5, 6],
    });

    assert_eq!("{\"key1\":[1,2,4,5,6]}", info.to_string());

    let saved: Vec<Option<Vec<u8>>> = vec![Some(vec![1, 2, 3]), Some(vec![4, 5, 6]), None];
    let info = json!({
        "key1": saved,
        "_key3":null,
        "key5": true,
    });
    assert_eq!(
        "{\"_key3\":null,\"key1\":[[1,2,3],[4,5,6],null],\"key5\":true}",
        info.to_string()
    );
}

#[test]
fn deserialize_vec() {
    let info = serde_json::from_str::<Vec<u8>>("[1,2,4,5,6]").unwrap();
    assert_eq!(vec![1, 2, 4, 5, 6], info);

    let info = serde_json::from_str::<Vec<Option<Vec<u8>>>>("[[1,2,3],[4,5,6],null]").unwrap();
    assert_eq!(vec![Some(vec![1, 2, 3]), Some(vec![4, 5, 6]), None], info);
}
