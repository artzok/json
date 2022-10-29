use std::{collections::HashMap, iter, fmt::Display};

use crate::{JsonBuilder, JsonValue, ToJson};

///
/// [`JsonValue::Object`] 内部数据存储类型
///
/// 其内部使用 [`HashMap<String, JsonValue>`] 形式存储键值对
///
#[derive(Debug)]
pub struct JsonObject {
    map: HashMap<String, JsonValue>,
}

impl JsonObject {

    pub fn new() -> JsonObject {
        JsonObject {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: JsonValue) {
        self.map.insert(key, value);
    }
}

impl JsonBuilder for JsonObject {

    fn build(&self, mut json: String, pretty: bool, level: usize, indent: &str) -> String {
        json.push('{');

        let last = self.map.len() - 1;
        let indents: String = iter::repeat(indent).take(level + 1).collect();

        for (index, (key, value)) in self.map.iter().enumerate() {
            // push indents
            if pretty {
                json.push('\n');
                json.push_str(&indents);
            }

            // push sep
            json.push_str(&format!("\"{}\":", key));

            if pretty {
                json.push(' ');
            }

            // push value
            json = value.build(json, pretty, level + 1, indent);

            // push ,
            if index < last {
                json.push(',');
            }
        }

        // push \n
        if pretty {
            json.push('\n');

            if level > 0 {
                let indents: String = iter::repeat(indent).take(level).collect();
                json.push_str(&indents);
            }
        }
        json.push('}');
        json
    }
}

impl Display for JsonObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build(String::new(), false, 0, ""))
    }
}

impl ToJson for JsonObject {

    fn pretty(&self) -> String {
        self.to_json(true, "| ")
    }

    fn to_json(&self, pretty: bool, indent: &str) -> String {
        self.build(String::new(), pretty, 0, indent)
    }
}
