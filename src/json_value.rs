use std::fmt::Display;

use crate::{utils, Error, ErrorKind, JsonArray, JsonBuilder, JsonObject, Result, ToJson};

///
/// All JSON element type.
///
/// **This class can coerce values to another type when requested.** 
/// 
/// 1. When the requested type is a [`bool`], strings will be coerced
/// using a case-insensitive comparison to "true" and "false".
///
/// 2. When the requested type is a integer, [`i128`] or [`u128`] will 
/// be coerced return type.
///
/// 3. When the requested type is an float, [`i128`] or [`u128`] or [`f64`] 
/// will be coerced return type.
///
/// **Note:** `as` opteration maybe loss of precision or error result, but 
/// the `json` library doesn't do any processing.
#[derive(Debug, Clone)]
pub enum JsonValue {
    /// Is `null` value(ignore case).
    Null,
    /// Is `bool` value, only equal `false` or `true`(ignore case).
    Bool(bool),
    /// Is all negative integer, use of `i128` type save value.
    Int(i128),
    /// Is all positive integer，use of `u128` type save value.
    Uint(u128),
    /// Is All float number, includings [`f32`] 和 [`f64`].
    Float(f64),
    /// Is a string value, all escape sequences have been escaped.
    String(String),
    /// Is nest JSON ojbect, internal use [`JsonObject`] save values.
    Object(JsonObject),
    /// Is nest JSON array, internal use [`JsonArray`] save values.
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

// for to string and print, internal use [`JsonBuilder`] implement.
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
    /// Convert to [`bool`] value.
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

    /// Convert to [`i8`] value.
    pub fn as_i8(&self) -> Result<i8> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Uint(value) => Ok(*value as i8),
            JsonValue::Int(value) => Ok(*value as i8),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need i8")),
        }
    }

    /// Convert to [`i16`] value.
    pub fn as_i16(&self) -> Result<i16> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Uint(value) => Ok(*value as i16),
            JsonValue::Int(value) => Ok(*value as i16),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need i16")),
        }
    }

    /// Convert to [`i32`] value.
    pub fn as_i32(&self) -> Result<i32> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Uint(value) => Ok(*value as i32),
            JsonValue::Int(value) => Ok(*value as i32),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need i32")),
        }
    }

    /// Convert to [`i64`] value.
    pub fn as_i64(&self) -> Result<i64> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Uint(value) => Ok(*value as i64),
            JsonValue::Int(value) => Ok(*value as i64),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need i64")),
        }
    }

    /// Convert to [`i128`] value.
    pub fn as_i128(&self) -> Result<i128> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Uint(value) => Ok(*value as i128),
            JsonValue::Int(value) => Ok(*value),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need i128")),
        }
    }

    /// Convert to [`u8`] value.
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

    /// Convert to [`u16`] value.
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

    /// Convert to [`u32`] value.
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

    /// Convert to [`u64`] value.
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

    /// Convert to [`u128`] value.
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

    /// Convert to [`f32`] value.
    pub fn as_f32(&self) -> Result<f32> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Uint(value) => Ok(*value as f32),
            JsonValue::Int(value) => Ok(*value as f32),
            JsonValue::Float(value) => Ok(*value as f32),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need uf32")),
        }
    }

    /// Convert to [`f64`] value.
    pub fn as_f64(&self) -> Result<f64> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Uint(value) => Ok(*value as f64),
            JsonValue::Int(value) => Ok(*value as f64),
            JsonValue::Float(value) => Ok(*value),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need uf64")),
        }
    }

    /// Convert to [`&str`] value.
    pub fn as_str(&self) -> Result<&str> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::String(str) => Ok(str),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need string")),
        }
    }

    /// Convert to [`&mut str`] value.
    pub fn as_mut_str(&mut self) -> Result<&mut str> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::String(str) => Ok(str),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need string")),
        }
    }

    /// Convert to [`String`] value.
    pub fn as_string(self) -> Result<String> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::String(str) => Ok(str),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need string")),
        }
    }

    /// Convert to [`&JsonObject`] value.
    pub fn as_object_ref(&self) -> Result<&JsonObject> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Object(object) => Ok(object),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need json object")),
        }
    }

    /// Convert to [`&mut JsonObject`] value.
    pub fn as_object_mut_ref(&mut self) -> Result<&mut JsonObject> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Object(object) => Ok(object),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need json object")),
        }
    }

    /// Convert to [`JsonObject`] value.
    pub fn as_object(self) -> Result<JsonObject> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Object(object) => Ok(object),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need json object")),
        }
    }

    /// Convert to [`&JsonArray`] value.
    pub fn as_array_ref(&self) -> Result<&JsonArray> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Array(array) => Ok(array),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need json array")),
        }
    }

    /// Convert to [`&mut JsonArray`] value.
    pub fn as_array_mut_ref(&mut self) -> Result<&mut JsonArray> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Array(array) => Ok(array),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need json array")),
        }
    }

    /// Convert to [`JsonArray`] value.
    pub fn as_array(self) -> Result<JsonArray> {
        match self {
            JsonValue::Null => Err(Error::new(ErrorKind::ValueNull, "value is null")),
            JsonValue::Array(array) => Ok(array),
            _ => Err(Error::new(ErrorKind::TypeNotMatch, "need json array")),
        }
    }

    /// Convert to bool.
    pub fn opt_bool(&self) -> Option<bool> {
        self.as_bool().ok()
    }

    /// Convert to i8.
    pub fn opt_i8(&self) -> Option<i8> {
        self.as_i8().ok()
    }

    /// Convert to i16.
    pub fn opt_i16(&self) -> Option<i16> {
        self.as_i16().ok()
    }

    /// Convert to i32.
    pub fn opt_i32(&self) -> Option<i32> {
        self.as_i32().ok()
    }

    /// Convert to i64.
    pub fn opt_i64(&self) -> Option<i64> {
        self.as_i64().ok()
    }

    /// Convert to i128.
    pub fn opt_i128(&self) -> Option<i128> {
        self.as_i128().ok()
    }

    /// Convert to u8.
    pub fn opt_u8(&self) -> Option<u8> {
        self.as_u8().ok()
    }

    /// Convert to u16.
    pub fn opt_u16(&self) -> Option<u16> {
        self.as_u16().ok()
    }

    /// Convert to u32.
    pub fn opt_u32(&self) -> Option<u32> {
        self.as_u32().ok()
    }

    /// Convert to u64.
    pub fn opt_u64(&self) -> Option<u64> {
        self.as_u64().ok()
    }

    /// Convert to u128.
    pub fn opt_u128(&self) -> Option<u128> {
        self.as_u128().ok()
    }

    /// Convert to f32.
    pub fn opt_f32(&self) -> Option<f32> {
        self.as_f32().ok()
    }

    /// Convert to f64.
    pub fn opt_f64(&self) -> Option<f64> {
        self.as_f64().ok()
    }

    /// Convert to &str.
    pub fn opt_str(&self) -> Option<&str> {
        self.as_str().ok()
    }

    /// Convert to &mut str.
    pub fn opt_mut_str(&mut self) -> Option<&mut str> {
        self.as_mut_str().ok()
    }

    /// Convert to String.
    pub fn opt_string(self) -> Option<String> {
        self.as_string().ok()
    }

    /// Convert to &JsonObject.
    pub fn opt_object_ref(&self) -> Option<&JsonObject> {
        self.as_object_ref().ok()
    }

    /// Convert to &mut JsonObject.
    pub fn opt_object_mut_ref(&mut self) -> Option<&mut JsonObject> {
        self.as_object_mut_ref().ok()
    }

    /// Convert to JsonObject.
    pub fn opt_object(self) -> Option<JsonObject> {
        self.as_object().ok()
    }

    /// Convert to &JsonArray.
    pub fn opt_array_ref(&self) -> Option<&JsonArray> {
        self.as_array_ref().ok()
    }

    /// Convert to &mut JsonArray.
    pub fn opt_array_mut_ref(&mut self) -> Option<&mut JsonArray> {
        self.as_array_mut_ref().ok()
    }

    /// Convert to JsonArray.
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
