use std::{fmt::Display, str::Chars};

use crate::{JsonArray, JsonBuilder, JsonObject, ToJson};

///
/// Json 所有元素类型
///
/// [`JsonValue::None`] 表示 'null' or 'NULL'(ignore case)
///
/// [`JsonValue::Bool`] 表示  'false' or 'true'(ignore case)
///
/// [`JsonValue::Int`] 表示所有负整数，其内部使用有符号 `i128` 类型存储值
///
/// [`JsonValue::Uint`] 表示所有正整数，其内部使用无符号 `u128` 类型存储
///
/// [`JsonValue::Int`] 和 `JsonValue::Uint` 只是为了简化解析过程，其内部存储
/// 类型并不代表最终类型，用户获取对应值时会根据返回值的类型取转换，比如：
/// 1. json 字符串中有一个正整数 128，解析后 `JsonValue` 会使用 [`JsonValue::Uint`]
/// 存储，但用户在访问时却可以调用 `get_i32` 方法获取该值，其内部将会自动转换，但是需要注意
/// 这里可能发生溢出
///
/// 2. json 中有一个负数 -128，解析后 `JsonValue` 会使用 [`JsonValue::Int`] 存储，此时
/// 如果用户通过 `get_u32` 方法获取该值时，则会发生 [`crate::ErrorKind::CastError`]
///
/// [`JsonValue::String`] 表示一个普通字符串值
///
/// [`JsonValue::Object`] 表示嵌套对象
///
/// [`JsonValue::Array`] 表示嵌套列表
///
#[derive(Debug)]
pub enum JsonValue {
    None,
    Bool(bool),
    Int(i128),
    Uint(u128),
    Float(f64),
    String(String),
    Object(JsonObject),
    Array(JsonArray),
}

impl JsonBuilder for JsonValue {
    fn build(&self, mut json: String, pretty: bool, level: usize, indent: &str) -> String {
        match self {
            JsonValue::None => json.push_str("null"),
            JsonValue::Bool(b) => json.push_str(&b.to_string()),
            JsonValue::Int(i) => json.push_str(&i.to_string()),
            JsonValue::Uint(u) => json.push_str(&u.to_string()),
            JsonValue::Float(d) => json.push_str(&d.to_string()),
            JsonValue::String(s) => json.push_str(&format!("\"{}\"", replace_escape(s))),
            JsonValue::Array(array) => {
                json = JsonBuilder::build(array, json, pretty, level, indent);
            }
            JsonValue::Object(object) => {
                json = JsonBuilder::build(object, json, pretty, level, indent);
            }
        }
        json
    }
}

impl Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build(String::new(), false, 0, ""))
    }
}

impl ToJson for JsonValue {
    fn pretty(&self) -> String {
        self.to_json(true, "| ")
    }

    fn to_json(&self, pretty: bool, indent: &str) -> String {
        self.build(String::new(), pretty, 0, indent)
    }
}

fn replace_escape(str: &str) -> String {
    str.chars().map(|ch| {
         match ch {
            '\\' => "\\\\".to_string(),
            '\"' => "\\\"".to_string(),
            '\x0C' => "\\f".to_string(),
            '\t' => "\\t".to_string(),
            '\n' => "\\n".to_string(),
            '\x08' => "\\b".to_string(),
            '\r' => "\\r".to_string(),
            ch =>  ch.to_string(),
        }
    }).collect()
}
