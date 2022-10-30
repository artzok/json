use std::{borrow::Borrow, collections::HashMap, fmt::Display, hash::Hash, iter};

use crate::{Error, ErrorKind, JsonBuilder, JsonValue, Result, ToJson};

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
    pub fn get_i64<Q :?Sized>(&self, key: &Q) -> Result<Option<i64>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq
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
    pub fn get_i128<Q :?Sized>(&self, key: &Q) -> Result<Option<i128>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq
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
    pub fn get_u64<Q :?Sized>(&self, key: &Q) -> Result<Option<u64>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq
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
    pub fn get_u128<Q :?Sized>(&self, key: &Q) -> Result<Option<u128>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq
    {
        match self.get(key) {
            None => Err(Error::new(ErrorKind::NotFound, "Not found u128 value")),
            Some(JsonValue::None) => Ok(None),
            Some(&JsonValue::Int(i)) => Ok(Some(i as u128)),
            Some(&JsonValue::Uint(u)) => Ok(Some(u)),
            _ => Err(Error::new(ErrorKind::CastError, "value type not integer")), 
        }
    }

    pub fn get_bool<Q :?Sized>(&self, _: &Q) -> Result<Option<bool>>
    where
        String: Borrow<Q>,
        Q: Hash + Eq
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
