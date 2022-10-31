use std::{borrow::Borrow, fmt::Display, hash::Hash, iter};

use linked_hash_map::LinkedHashMap;

use crate::{
    tokener::JsonTokener, utils, Error, ErrorKind, JsonArray, JsonBuilder, JsonValue, Result,
    ToJson,
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
    map: LinkedHashMap<String, JsonValue>,
}

impl JsonObject {
    /// Create an empty [`JsonObject`].
    pub fn new() -> JsonObject {
        JsonObject {
            map: LinkedHashMap::new(),
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
        self.map.len()
    }

    /// Check empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Maps `key` to `value`, clobbering any existing
    /// key/value mapping with the same name.
    pub fn insert<K: Into<String>, V: Into<JsonValue>>(&mut self, key: K, value: V) {
        self.map.insert(key.into(), value.into());
    }

    /// Get value borrow of `key`.
    /// Return [`None`] if not found value of `key`.
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&JsonValue>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.get(key)
    }

    /// Get value mut borrow of `key`.
    /// Return [`None`] if not found value of `key`.
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut JsonValue>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.get_mut(key)
    }

    /// Remove and return value of `key`.
    /// Return [`None`] if not found value of `key`.
    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<JsonValue>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.remove(key)
    }

    /// Appends `value` to the array already mapped to `name`. If
    /// this object has no mapping for `name`, this inserts
    ///  a new mapping.
    pub fn accumulate<T: Into<JsonValue>>(&mut self, key: String, value: T) {
        match self.remove(&key) {
            None => self.insert(key, value),
            Some(mut current) => match current {
                JsonValue::Array(ref mut array) => {
                    array.push(value.into());
                    self.insert(key, current);
                }
                _ => {
                    let mut array = JsonArray::new();
                    array.push(current);
                    array.push(value.into());
                    self.insert(key, JsonValue::Array(array));
                }
            },
        }
    }
}

impl JsonBuilder for JsonObject {
    fn build(&self, mut json: String, pretty: bool, level: usize, indent: &str) -> String {
        json.push('{');

        let last = if self.is_empty() { 0 } else { self.len() - 1 };
        let indents: String = iter::repeat(indent).take(level + 1).collect();

        for (index, (key, value)) in self.map.iter().enumerate() {
            // push indents
            if pretty {
                json.push('\n');
                json.push_str(&indents);
            }

            // push sep
            json.push('\"');
            json.push_str(&utils::replace_escape(key));
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
