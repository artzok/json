use std::{iter, fmt::Display};

use crate::{JsonBuilder, JsonValue, ToJson};

///
/// [`JsonValue::Array`] 内部数据存储类型
///
/// 其内部以 [`Vec<JsonValue>`] 形式存储数组
///
#[derive(Debug)]
pub struct JsonArray {
    list: Vec<JsonValue>,
}

impl JsonArray {

    pub fn new() -> JsonArray {
        JsonArray { list: vec![] }
    }

    pub fn push(&mut self, value: JsonValue) {
        self.list.push(value);
    }
}

impl JsonBuilder for JsonArray {

    fn build(&self, mut json: String, pretty: bool, level: usize, indent: &str) -> String {
        json.push('[');

        let last = self.list.len() - 1;
        let indents: String = iter::repeat(indent).take(level + 1_usize).collect();

        for (index, item) in self.list.iter().enumerate() {
            // push \n
            if pretty {
                json.push('\n');
                // push indent
                json.push_str(&indents);
            }

            // push value
            json = item.build(json, pretty, level + 1, indent);

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
        
        json.push(']');
        json
    }
}

impl Display for JsonArray {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build(String::new(), false, 0, ""))
    }
}

impl ToJson for JsonArray {
    fn pretty(&self) ->String {
        self.to_json(true, "| ")
    }

    fn to_json(&self, pretty: bool, indent: &str) -> String {
        self.build(String::new(), pretty, 0, indent)
    }
}