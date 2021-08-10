use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::Value;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    a: String,
    b: i32
}

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    pub name: String,

    #[serde(rename="$value")]
    pub items: Vec<Item>,

    // #[serde(flatten)]
    // extra: HashMap<String, Value>
}

fn value_to_xml(value: &Value) -> Result<String> {
    match value {
        Value::Null => Ok("".to_string()),
        Value::Bool(b) => Ok(b.to_string()),
        Value::Number(n) => Ok(n.to_string()),
        Value::String(s) => Ok(s.clone()),
        Value::Array(a) => array_to_xml(a),
        Value::Object(o) => obj_to_xml(o),
    }
}

fn array_to_xml(a: &Vec<Value>) -> Result<String> {
    let mut s: String = "".to_string();
    for v in a {
        s.push_str(value_to_xml(v)?.as_str());
    }

    Ok(s)
}

fn obj_to_xml(obj: &serde_json::Map<String, Value>) -> Result<String> {
    let mut s: String = "".to_string();

    for (key, value) in obj {
        s.push_str(format!("<{}>{}</{}>", key, value_to_xml(value)?, key).as_str());
    }

    Ok(s)
}

fn main() -> anyhow::Result<()> {
    let data = r#"
        {
            "name": "Vasya",
            "items": [
                { "a": "test", "b": 25 },
                { "a": "jest", "b": 77 }
            ],
            "kobra": "fdsfdsfsdfdsf",
            "tvers": [
                { "rrr": "x" },
                { "vvv": "y" }
            ],
            "qwert": {
                "cvcvc" : 1,
                "sdsdfsdf": true
            }
        }
    "#;

    let t: serde_json::Map<String, Value> = serde_json::from_str(data)?;

    let xml = format!("<div>{}</div>", obj_to_xml(&t)?);

    println!("{}", xml);

    Ok(())
}
