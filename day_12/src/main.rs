use serde_json::{Map, Value};
use std::fs;

fn main() {
    println!("Hello, world!");

    let contents: Value = serde_json::from_str(&fs::read_to_string("input.json").unwrap()).unwrap();

    let mut total: i64 = 0;

    recurse_data(&contents, &mut total);

    println!("TOTAL: {}", total);
}

fn recurse_data(data: &Value, total: &mut i64) {
    match data {
        Value::Number(nbr) => {
            *total += nbr.as_i64().unwrap();
        }
        Value::Array(values) => {
            for value in values {
                recurse_data(&value, total);
            }
        }
        Value::Object(object) => {
            if !validate_object(object){
                return;
            };
            for value in object.values() {
                recurse_data(&value, total);
            }
        }
        _ => (),
    }
}

/// Check the immediate values of an object;
/// reject the object is any immediate value == 'red'
fn validate_object(object: &Map<String, Value>) -> bool {
    for value in object.values() {
        if let Value::String(string) = value {
            if let "red" = string.as_str() {
                return false;
            }
        }
    }
    return true;
}
