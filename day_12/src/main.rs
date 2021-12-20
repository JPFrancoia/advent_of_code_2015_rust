use serde_json::{Result, Value};
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("Hello, world!");

    // let contents: HashMap<String, Value> = serde_json::from_str(&fs::read_to_string("input.json").unwrap()).unwrap();
    let contents: Value = serde_json::from_str(&fs::read_to_string("input.json").unwrap()).unwrap();

    let john = serde_json::json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ],
        "potatoes": 42,
        "nested":
            {"name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ],
            "potatoes": 42}
    });


    // let john = serde_json::json!([-1,{"a":1}]);

    let mut total: i64 = 0;

    // recurse_data(&john["age"]);
    // recurse_data(&john, &mut total);
    recurse_data(&contents, &mut total);

    println!("TOTAL: {}", total);
}

fn recurse_data(data: &Value, total: &mut i64) -> i64 {
    match data {
        Value::String(string) => println!("{}", string),
        Value::Number(nbr) => {
            println!("{}", nbr);
            *total += nbr.as_i64().unwrap();
        }
        Value::Array(values) => {
            for value in values {
                recurse_data(&value, total);
            }
        }
        Value::Object(object) => {
            for (key, value) in object {
                recurse_data(&value, total);
            }
        }
        _ => println!("other"),
    }

    return 0;
}
