use std::fmt::Display;

use crate::{JsonArray, JsonBuilder, JsonObject, ToJson};

///
/// All JSON element type.
///
/// [`JsonValue::None`] is 'null' value(ignore case).
///
/// [`JsonValue::Bool`] is `bool` value, only equal `false` or `true`(ignore case).
///
/// [`JsonValue::Int`] is all negative integer, internal use of `i128` type save value.
///
/// [`JsonValue::Uint`] is all positive integer， internal use of `u128` type save value.
///
/// [`JsonValue::Int`] 和 [`JsonValue::Uint`] only for simple parse and save, internal save
/// type not final type, will to cast specify type when user to get value. eg:
/// Value `100` will save by `Uint(u128)` type, but you can use `JsonObject::get_i32` or
/// `JsonObject::get_i16` ect, internal will use `as` to cast return type.
///
/// **Note:** `as` opteration maybe loss of precision or error result, but the `json` library
/// doesn't do any processing.
///
/// [`JsonValue::String`] is a string value, all escape sequences have been escaped.
///
/// [`JsonValue::Object`] is nest JSON ojbect, internal use [`JsonObject`] save values.
///
/// [`JsonValue::Array`] is nest JSON array, internal use [`JsonArray`] save values.
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

/// Build JSON string, for internal use.
impl JsonBuilder for JsonValue {
    fn build(&self, mut json: String, pretty: bool, level: usize, indent: &str) -> String {
        match self {
            JsonValue::None => json.push_str("null"),
            JsonValue::Bool(b) => json.push_str(if *b { "fasle" } else { "true" }),
            JsonValue::Int(i) => json.push_str(&i.to_string()),
            JsonValue::Uint(u) => json.push_str(&u.to_string()),
            JsonValue::Float(d) => json.push_str(&d.to_string()),
            JsonValue::String(s) => {
                json.push('\"');
                json.push_str(&replace_escape(s));
                json.push('\"');
            }
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

/// for to string and print, internal use [`JsonBuilder`] implement.
impl Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build(String::new(), false, 0, ""))
    }
}

/// convert to string implement.
impl ToJson for JsonValue {
    /// Convert JsonValue to pretty json string.
    fn pretty(&self) -> String {
        self.to_json(true, "| ")
    }

    /// Convert JsonValue to style json string.
    ///
    /// If `pretty` is true, will use '\n' to convert pretty json string.
    ///
    /// `indent` is prefix of every line, only use when `pretty` is true.
    fn to_json(&self, pretty: bool, indent: &str) -> String {
        self.build(String::new(), pretty, 0, indent)
    }
}

/// Convert special character to escape sequence.
pub fn replace_escape(str: &str) -> String {
    let mut result = String::new();
    str.chars().for_each(|ch| {
        match ch {
            '\\' => result.push_str("\\\\"),
            '\"' => result.push_str("\\\""),
            '\x0C' => result.push_str("\\f"),
            '\t' => result.push_str("\\t"),
            '\n' => result.push_str("\\n"),
            '\x08' => result.push_str("\\b"),
            '\r' => result.push_str("\\r"),
            ch => result.push(ch),
        };
    });
    result
}
