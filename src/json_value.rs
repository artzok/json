use std::fmt::Display;

use crate::{utils, Error, ErrorKind, JsonArray, JsonBuilder, JsonObject, Result, ToJson};

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
#[derive(Debug, Clone)]
pub enum JsonValue {
    Null,
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
            JsonValue::Null => json.push_str("null"),
            JsonValue::Bool(b) => json.push_str(if *b { "false" } else { "true" }),
            JsonValue::Int(i) => json.push_str(&i.to_string()),
            JsonValue::Uint(u) => json.push_str(&u.to_string()),
            JsonValue::Float(d) => json.push_str(&d.to_string()),
            JsonValue::String(s) => {
                json.push('\"');
                json.push_str(&utils::replace_escape(s));
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

impl JsonValue {
    pub fn as_bool(&self) -> Result<bool> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Bool(value) => Ok(*value),
            JsonValue::String(value) => {
                if value.eq_ignore_ascii_case("true") {
                    Ok(true)
                } else if value.eq_ignore_ascii_case("false") {
                    Ok(false)
                } else {
                    Err(Error::new(
                        ErrorKind::TypeNotMatch,
                        "need bool but get string",
                    ))
                }
            }
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need bool")),
        }
    }

    pub fn as_i8(&self) -> Result<i8> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Uint(value) => Ok(*value as i8),
            JsonValue::Int(value) => Ok(*value as i8),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need i8")),
        }
    }

    pub fn as_i16(&self) -> Result<i16> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Uint(value) => Ok(*value as i16),
            JsonValue::Int(value) => Ok(*value as i16),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need i16")),
        }
    }

    pub fn as_i32(&self) -> Result<i32> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Uint(value) => Ok(*value as i32),
            JsonValue::Int(value) => Ok(*value as i32),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need i32")),
        }
    }

    pub fn as_i64(&self) -> Result<i64> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Uint(value) => Ok(*value as i64),
            JsonValue::Int(value) => Ok(*value as i64),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need i64")),
        }
    }

    pub fn as_i128(&self) -> Result<i128> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Uint(value) => Ok(*value as i128),
            JsonValue::Int(value) => Ok(*value),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need i128")),
        }
    }

    pub fn as_u8(&self) -> Result<u8> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Uint(value) => Ok(*value as u8),
            JsonValue::Int(value) => {
                if *value > 0 {
                    Ok(*value as u8)
                } else {
                    Err(Error::new(
                        ErrorKind::TypeNotMatch,
                        "need u8 but get less than 0",
                    ))
                }
            }
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need u8")),
        }
    }

    pub fn as_u16(&self) -> Result<u16> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Uint(value) => Ok(*value as u16),
            JsonValue::Int(value) => {
                if *value > 0 {
                    Ok(*value as u16)
                } else {
                    Err(Error::new(
                        ErrorKind::TypeNotMatch,
                        "need u16 but get less than 0",
                    ))
                }
            }
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need u16")),
        }
    }

    pub fn as_u32(&self) -> Result<u32> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Uint(value) => Ok(*value as u32),
            JsonValue::Int(value) => {
                if *value > 0 {
                    Ok(*value as u32)
                } else {
                    Err(Error::new(
                        ErrorKind::TypeNotMatch,
                        "need u32 but get less than 0",
                    ))
                }
            }
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need u32")),
        }
    }

    pub fn as_u64(&self) -> Result<u64> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Uint(value) => Ok(*value as u64),
            JsonValue::Int(value) => {
                if *value > 0 {
                    Ok(*value as u64)
                } else {
                    Err(Error::new(
                        ErrorKind::TypeNotMatch,
                        "need u64 but get less than 0",
                    ))
                }
            }
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need u64")),
        }
    }

    pub fn as_u128(&self) -> Result<u128> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Uint(value) => Ok(*value as u128),
            JsonValue::Int(value) => {
                if *value > 0 {
                    Ok(*value as u128)
                } else {
                    Err(Error::new(
                        ErrorKind::TypeNotMatch,
                        "need u128 but get less than 0",
                    ))
                }
            }
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need u128")),
        }
    }

    pub fn as_f32(&self) -> Result<f32> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Uint(value) => Ok(*value as f32),
            JsonValue::Int(value) => Ok(*value as f32),
            JsonValue::Float(value) => Ok(*value as f32),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need uf32")),
        }
    }

    pub fn as_f64(&self) -> Result<f64> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Uint(value) => Ok(*value as f64),
            JsonValue::Int(value) => Ok(*value as f64),
            JsonValue::Float(value) => Ok(*value),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need uf64")),
        }
    }

    pub fn as_str(&self) -> Result<&str> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::String(str) => Ok(str),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need string")),
        }
    }

    pub fn as_mut_str(&mut self) -> Result<&mut str> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::String(str) => Ok(str),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need string")),
        }
    }

    pub fn as_string(self) -> Result<String> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::String(str) => Ok(str),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need string")),
        }
    }

    pub fn as_object_ref(&self) -> Result<&JsonObject> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Object(object) => Ok(object),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need json object")),
        }
    }

    pub fn as_object_mut_ref(&mut self) -> Result<&mut JsonObject> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Object(object) => Ok(object),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need json object")),
        }
    }

    pub fn as_object(self) -> Result<JsonObject> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Object(object) => Ok(object),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need json object")),
        }
    }

    pub fn as_array_ref(&self) -> Result<&JsonArray> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Array(array) => Ok(array),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need json array")),
        }
    }

    pub fn as_array_mut_ref(&mut self) -> Result<&mut JsonArray> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Array(array) => Ok(array),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need json array")),
        }
    }

    pub fn as_array(self) -> Result<JsonArray> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Array(array) => Ok(array),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need json array")),
        }
    }

    pub fn opt_bool(&self) -> Option<bool> {
        self.as_bool().ok()
    }

    pub fn opt_i8(&self) -> Option<i8> {
        self.as_i8().ok()
    }

    pub fn opt_i16(&self) -> Option<i16> {
        self.as_i16().ok()
    }

    pub fn opt_i32(&self) -> Option<i32> {
        self.as_i32().ok()
    }

    pub fn opt_i64(&self) -> Option<i64> {
        self.as_i64().ok()
    }

    pub fn opt_i128(&self) -> Option<i128> {
        self.as_i128().ok()
    }

    pub fn opt_u8(&self) -> Option<u8> {
        self.as_u8().ok()
    }

    pub fn opt_u16(&self) -> Option<u16> {
        self.as_u16().ok()
    }

    pub fn opt_u32(&self) -> Option<u32> {
        self.as_u32().ok()
    }

    pub fn opt_u64(&self) -> Option<u64> {
        self.as_u64().ok()
    }

    pub fn opt_u128(&self) -> Option<u128> {
        self.as_u128().ok()
    }

    pub fn opt_f32(&self) -> Option<f32> {
        self.as_f32().ok()
    }

    pub fn opt_f64(&self) -> Option<f64> {
        self.as_f64().ok()
    }

    pub fn opt_str(&self) -> Option<&str> {
        self.as_str().ok()
    }

    pub fn opt_mut_str(&mut self) -> Option<&mut str> {
        self.as_mut_str().ok()
    }

    pub fn opt_string(self) -> Option<String> {
        self.as_string().ok()
    }

    pub fn opt_object_ref(&self) -> Option<&JsonObject> {
        self.as_object_ref().ok()
    }

    pub fn opt_object_mut_ref(&mut self) -> Option<&mut JsonObject> {
        self.as_object_mut_ref().ok()
    }

    pub fn opt_object(self) -> Option<JsonObject> {
        self.as_object().ok()
    }

    pub fn opt_array_ref(&self) -> Option<&JsonArray> {
        self.as_array_ref().ok()
    }

    pub fn opt_array_mut_ref(&mut self) -> Option<&mut JsonArray> {
        self.as_array_mut_ref().ok()
    }

    pub fn opt_array(self) -> Option<JsonArray> {
        self.as_array().ok()
    }
}

impl From<bool> for JsonValue {
    fn from(v: bool) -> Self {
        JsonValue::Bool(v)
    }
}

impl From<&str> for JsonValue {
    fn from(v: &str) -> Self {
        JsonValue::String(v.to_string())
    }
}

impl From<String> for JsonValue {
    fn from(v: String) -> Self {
        JsonValue::String(v)
    }
}

impl From<i8> for JsonValue {
    fn from(value: i8) -> Self {
        if value > 0 {
            JsonValue::Uint(value as u128)
        } else {
            JsonValue::Int(value.into())
        }
    }
}

impl From<i16> for JsonValue {
    fn from(value: i16) -> Self {
        if value > 0 {
            JsonValue::Uint(value as u128)
        } else {
            JsonValue::Int(value.into())
        }
    }
}

impl From<i32> for JsonValue {
    fn from(value: i32) -> Self {
        if value > 0 {
            JsonValue::Uint(value as u128)
        } else {
            JsonValue::Int(value.into())
        }
    }
}

impl From<i64> for JsonValue {
    fn from(value: i64) -> Self {
        if value > 0 {
            JsonValue::Uint(value as u128)
        } else {
            JsonValue::Int(value.into())
        }
    }
}

impl From<i128> for JsonValue {
    fn from(value: i128) -> Self {
        if value > 0 {
            JsonValue::Uint(value as u128)
        } else {
            JsonValue::Int(value)
        }
    }
}

impl From<u8> for JsonValue {
    fn from(value: u8) -> Self {
        JsonValue::Uint(value.into())
    }
}

impl From<u16> for JsonValue {
    fn from(value: u16) -> Self {
        JsonValue::Uint(value.into())
    }
}

impl From<u32> for JsonValue {
    fn from(value: u32) -> Self {
        JsonValue::Uint(value.into())
    }
}

impl From<u64> for JsonValue {
    fn from(value: u64) -> Self {
        JsonValue::Uint(value.into())
    }
}

impl From<u128> for JsonValue {
    fn from(value: u128) -> Self {
        JsonValue::Uint(value)
    }
}

impl From<f32> for JsonValue {
    fn from(value: f32) -> Self {
        JsonValue::Float(value.into())
    }
}

impl From<f64> for JsonValue {
    fn from(value: f64) -> Self {
        JsonValue::Float(value)
    }
}

impl From<JsonObject> for JsonValue {
    fn from(value: JsonObject) -> Self {
        JsonValue::Object(value)
    }
}

impl From<JsonArray> for JsonValue {
    fn from(value: JsonArray) -> Self {
        JsonValue::Array(value)
    }
}
