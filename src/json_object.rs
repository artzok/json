use std::{borrow::Borrow, collections::HashMap, fmt::Display, hash::Hash, iter, array};

use crate::{
    json_value, tokener::JsonTokener, Error, ErrorKind, JsonBuilder, JsonValue, Result, ToJson, JsonArray,
};

/// A modifiable set of name/value mappings. Names are unique, non-null strings.
///  Values may be any mix of [`JsonValue`].
///
/// **This class can coerce values to another type when requested.**
///
/// 1. When the requested type is a [`bool`], strings will be coerced
/// using a case-insensitive comparison to "true" and "false".
///
/// 2. When the requested type is a integer, [`i128`] or [`u128`] will be coerced return type.
///
/// 3. When the requested type is an float, [`i128`] or [`u128`] or [`f64`] will be coerced return type.
#[derive(Debug, Clone)]
pub struct JsonObject {
    map: HashMap<String, JsonValue>,
}

impl JsonObject {
    /// Create an empty [`JsonObject`].
    pub fn new() -> JsonObject {
        JsonObject {
            map: HashMap::new(),
        }
    }

    /// Parse `json` to [`JsonObject`].
    ///
    /// Return [`ErrorKind::TypeNotMatch`] if the parsed result
    /// is not a [`JsonValue::Object`].
    pub fn create(json: &str) -> Result<JsonObject> {
        let json_value = JsonTokener::new(json).next_value()?;
        if let JsonValue::Object(jo) = json_value {
            Ok(jo)
        } else {
            Err(Error::new(
                ErrorKind::TypeNotMatch,
                "Need JsonValue::Object but not.",
            ))
        }
    }

    /// Returns the number of key/value mappings in this object.
    pub fn len(&self) -> usize {
        return self.map.len();
    }

    /// Maps `key` to `value`, clobbering any existing
    /// key/value mapping with the same name.
    pub fn insert(&mut self, key: String, value: JsonValue) {
        self.map.insert(key, value);
    }

    /// Maps `key` to `value`, clobbering any existing
    /// key/value mapping with the same name.
    pub fn insert_bool(&mut self, key: String, value: bool) {
        self.map.insert(key, JsonValue::Bool(value));
    }

    /// Maps `key` to `value`, clobbering any existing
    /// key/value mapping with the same name.
    pub fn insert_i8(&mut self, key: String, value: i8) {
        let value = if value > 0 {
            JsonValue::Uint(value as u128)
        } else {
            JsonValue::Int(value as i128)
        };
        self.insert(key, value);
    }

    /// Maps `key` to `value`, clobbering any existing
    /// key/value mapping with the same name.
    pub fn insert_i16(&mut self, key: String, value: i16) {
        let value = if value > 0 {
            JsonValue::Uint(value as u128)
        } else {
            JsonValue::Int(value as i128)
        };
        self.insert(key, value);
    }

    /// Maps `key` to `value`, clobbering any existing
    /// key/value mapping with the same name.
    pub fn insert_i32(&mut self, key: String, value: i32) {
        let value = if value > 0 {
            JsonValue::Uint(value as u128)
        } else {
            JsonValue::Int(value as i128)
        };
        self.insert(key, value);
    }

    /// Maps `key` to `value`, clobbering any existing
    /// key/value mapping with the same name.
    pub fn insert_i64(&mut self, key: String, value: i64) {
        let value = if value > 0 {
            JsonValue::Uint(value as u128)
        } else {
            JsonValue::Int(value as i128)
        };
        self.insert(key, value);
    }

    /// Maps `key` to `value`, clobbering any existing
    /// key/value mapping with the same name.
    pub fn insert_i128(&mut self, key: String, value: i128) {
        let value = if value > 0 {
            JsonValue::Uint(value as u128)
        } else {
            JsonValue::Int(value)
        };
        self.insert(key, value);
    }

 /// Maps `key` to `value`, clobbering any existing
    /// key/value mapping with the same name.
    pub fn insert_u8(&mut self, key: String, value: u8) {
        self.insert(key, JsonValue::Uint(value as u128));
    }

    /// Maps `key` to `value`, clobbering any existing
    /// key/value mapping with the same name.
    pub fn insert_u16(&mut self, key: String, value: u16) {
        self.insert(key, JsonValue::Uint(value as u128));
    }

    /// Maps `key` to `value`, clobbering any existing
    /// key/value mapping with the same name.
    pub fn insert_u32(&mut self, key: String, value: u32) {
        self.insert(key, JsonValue::Uint(value as u128));
    }

    /// Maps `key` to `value`, clobbering any existing
    /// key/value mapping with the same name.
    pub fn insert_u64(&mut self, key: String, value: u64) {
        self.insert(key, JsonValue::Uint(value as u128));
    }

    /// Maps `key` to `value`, clobbering any existing
    /// key/value mapping with the same name.
    pub fn insert_u128(&mut self, key: String, value: u128) {
        self.insert(key, JsonValue::Uint(value));
    }

    /// Maps `key` to `value`, clobbering any existing
    /// key/value mapping with the same name.
    pub fn insert_f32(&mut self, key: String, value: f32) {
        self.insert(key, JsonValue::Float(value as f64));
    }

    /// Maps `key` to `value`, clobbering any existing
    /// key/value mapping with the same name.
    pub fn insert_f64(&mut self, key: String, value: f64) {
        self.insert(key, JsonValue::Float(value));
    }

    /// Maps `key` to `value`, clobbering any existing
    /// key/value mapping with the same name.
    pub fn insert_string(&mut self, key: String, value: String) {
        self.insert(key, JsonValue::String(value));
    }

    /// Maps `key` to `value`, clobbering any existing
    /// key/value mapping with the same name.
    pub fn insert_object(&mut self, key: String, object: JsonObject) {
        self.insert(key, JsonValue::Object(object));
    }

    /// Maps `key` to `value`, clobbering any existing
    /// key/value mapping with the same name.
    pub fn insert_array(&mut self, key: String, value: JsonArray) {
        self.insert(key, JsonValue::Array(value));
    }

    /// Appends `value` to the array already mapped to `name`. If
    /// this object has no mapping for `name`, this inserts
    ///  a new mapping.
    pub fn accumulate(&mut self, key: String, value: JsonValue) {
        match self.remove(&key) {
            None => {
                self.insert(key, value)
            },
            Some(mut current) => {
                match current {
                     JsonValue::Array(ref mut array) => {
                         array.push(value);
                        self.insert(key, current);
                    },
                    _ => {
                        let mut array = JsonArray::new();
                        array.push(current);
                        array.push(value);
                        self.insert(key, JsonValue::Array(array));
                    }
                }
            }
        }
    }

    ///
    /// 如果不存在返回  not found error
    /// 如果类型不匹配返回  cast error
    /// 如果对应 key 的值是 null 则返回 Ok(None)
    /// 如果对应 key 的值是 i28 or u128, will to convert by as
    /// 否则返回 Ok(value)
    pub fn get_i8<Q: ?Sized>(&self, key: &Q) -> Result<Option<i8>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.get(key) {
            None => Err(Error::new(ErrorKind::NotFound, "Not found i8 value")),
            Some(JsonValue::None) => Ok(None),
            Some(&JsonValue::Int(i)) => Ok(Some(i as i8)),
            Some(&JsonValue::Uint(u)) => Ok(Some(u as i8)),
            _ => Err(Error::new(ErrorKind::CastError, "value type not integer")),
        }
    }

    ///
    /// 如果不存在返回  not found error
    /// 如果类型不匹配返回  cast error
    /// 如果对应 key 的值是 null 则返回 Ok(None)
    /// 如果对应 key 的值是 i28 or u128, will to convert by as
    /// 否则返回 Ok(value)
    pub fn get_i16<Q: ?Sized>(&self, key: &Q) -> Result<Option<i16>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.get(key) {
            None => Err(Error::new(ErrorKind::NotFound, "Not found i16 value")),
            Some(JsonValue::None) => Ok(None),
            Some(&JsonValue::Int(i)) => Ok(Some(i as i16)),
            Some(&JsonValue::Uint(u)) => Ok(Some(u as i16)),
            _ => Err(Error::new(ErrorKind::CastError, "value type not integer")),
        }
    }

    ///
    /// 如果不存在返回  not found error
    /// 如果类型不匹配返回  cast error
    /// 如果对应 key 的值是 null 则返回 Ok(None)
    /// 如果对应 key 的值是 i28 or u128, will to convert by as
    /// 否则返回 Ok(value)
    pub fn get_i32<Q: ?Sized>(&self, key: &Q) -> Result<Option<i32>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.get(key) {
            None => Err(Error::new(ErrorKind::NotFound, "Not found i32 value")),
            Some(JsonValue::None) => Ok(None),
            Some(&JsonValue::Int(i)) => Ok(Some(i as i32)),
            Some(&JsonValue::Uint(u)) => Ok(Some(u as i32)),
            _ => Err(Error::new(ErrorKind::CastError, "value type not integer")),
        }
    }

    ///
    /// 如果不存在返回  not found error
    /// 如果类型不匹配返回  cast error
    /// 如果对应 key 的值是 null 则返回 Ok(None)
    /// 如果对应 key 的值是 i28 or u128, will to convert by as
    /// 否则返回 Ok(value)
    pub fn get_i64<Q: ?Sized>(&self, key: &Q) -> Result<Option<i64>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.get(key) {
            None => Err(Error::new(ErrorKind::NotFound, "Not found i64 value")),
            Some(JsonValue::None) => Ok(None),
            Some(&JsonValue::Int(i)) => Ok(Some(i as i64)),
            Some(&JsonValue::Uint(u)) => Ok(Some(u as i64)),
            _ => Err(Error::new(ErrorKind::CastError, "value type not integer")),
        }
    }

    ///
    /// 如果不存在返回  not found error
    /// 如果类型不匹配返回  cast error
    /// 如果对应 key 的值是 null 则返回 Ok(None)
    /// 如果对应 key 的值是 i28 or u128, will to convert by as
    /// 否则返回 Ok(value)
    pub fn get_i128<Q: ?Sized>(&self, key: &Q) -> Result<Option<i128>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.get(key) {
            None => Err(Error::new(ErrorKind::NotFound, "Not found i128 value")),
            Some(JsonValue::None) => Ok(None),
            Some(&JsonValue::Int(i)) => Ok(Some(i)),
            Some(&JsonValue::Uint(u)) => Ok(Some(u as i128)),
            _ => Err(Error::new(ErrorKind::CastError, "value type not integer")),
        }
    }

    ///
    /// 如果不存在返回  not found error
    /// 如果类型不匹配返回  cast error
    /// 如果对应 key 的值是 null 则返回 Ok(None)
    /// 如果对应 key 的值是 i28 or u128, will to convert by as
    /// 否则返回 Ok(value)
    pub fn get_u8<Q: ?Sized>(&self, key: &Q) -> Result<Option<u8>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.get(key) {
            None => Err(Error::new(ErrorKind::NotFound, "Not found u8 value")),
            Some(JsonValue::None) => Ok(None),
            Some(&JsonValue::Int(i)) => Ok(Some(i as u8)),
            Some(&JsonValue::Uint(u)) => Ok(Some(u as u8)),
            _ => Err(Error::new(ErrorKind::CastError, "value type not integer")),
        }
    }

    ///
    /// 如果不存在返回  not found error
    /// 如果类型不匹配返回  cast error
    /// 如果对应 key 的值是 null 则返回 Ok(None)
    /// 如果对应 key 的值是 i28 or u128, will to convert by as
    /// 否则返回 Ok(value)
    pub fn get_u16<Q: ?Sized>(&self, key: &Q) -> Result<Option<u16>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.get(key) {
            None => Err(Error::new(ErrorKind::NotFound, "Not found u16 value")),
            Some(JsonValue::None) => Ok(None),
            Some(&JsonValue::Int(i)) => Ok(Some(i as u16)),
            Some(&JsonValue::Uint(u)) => Ok(Some(u as u16)),
            _ => Err(Error::new(ErrorKind::CastError, "value type not integer")),
        }
    }

    ///
    /// 如果不存在返回  not found error
    /// 如果类型不匹配返回  cast error
    /// 如果对应 key 的值是 null 则返回 Ok(None)
    /// 如果对应 key 的值是 i28 or u128, will to convert by as
    /// 否则返回 Ok(value)
    pub fn get_u32<Q: ?Sized>(&self, key: &Q) -> Result<Option<u32>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.get(key) {
            None => Err(Error::new(ErrorKind::NotFound, "Not found u32 value")),
            Some(JsonValue::None) => Ok(None),
            Some(&JsonValue::Int(i)) => Ok(Some(i as u32)),
            Some(&JsonValue::Uint(u)) => Ok(Some(u as u32)),
            _ => Err(Error::new(ErrorKind::CastError, "value type not integer")),
        }
    }

    ///
    /// 如果不存在返回  not found error
    /// 如果类型不匹配返回  cast error
    /// 如果对应 key 的值是 null 则返回 Ok(None)
    /// 如果对应 key 的值是 i28 or u128, will to convert by as
    /// 否则返回 Ok(value)
    pub fn get_u64<Q: ?Sized>(&self, key: &Q) -> Result<Option<u64>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.get(key) {
            None => Err(Error::new(ErrorKind::NotFound, "Not found u64 value")),
            Some(JsonValue::None) => Ok(None),
            Some(&JsonValue::Int(i)) => Ok(Some(i as u64)),
            Some(&JsonValue::Uint(u)) => Ok(Some(u as u64)),
            _ => Err(Error::new(ErrorKind::CastError, "value type not integer")),
        }
    }

    ///
    /// 如果不存在返回  not found error
    /// 如果类型不匹配返回  cast error
    /// 如果对应 key 的值是 null 则返回 Ok(None)
    /// 如果对应 key 的值是 i28 or u128, will to convert by as
    /// 否则返回 Ok(value)
    pub fn get_u128<Q: ?Sized>(&self, key: &Q) -> Result<Option<u128>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.get(key) {
            None => Err(Error::new(ErrorKind::NotFound, "Not found u128 value")),
            Some(JsonValue::None) => Ok(None),
            Some(&JsonValue::Int(i)) => Ok(Some(i as u128)),
            Some(&JsonValue::Uint(u)) => Ok(Some(u)),
            _ => Err(Error::new(ErrorKind::CastError, "value type not integer")),
        }
    }

    pub fn get_bool<Q: ?Sized>(&self, _: &Q) -> Result<Option<bool>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        Ok(Some(false))
    }

    fn get<Q: ?Sized>(&self, key: &Q) -> Option<&JsonValue>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.get(key)
    }

    fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut JsonValue>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.get_mut(key)
    }

    fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<JsonValue> 
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.remove(key)
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
            json.push('\"');
            json.push_str(&json_value::replace_escape(key));
            json.push_str("\":");

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
