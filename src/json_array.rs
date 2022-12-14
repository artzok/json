use std::{fmt::Display, iter};

use crate::{tokener::JsonTokener, Error, ErrorKind, JsonBuilder, JsonValue, Result, ToJson};

/// A dense indexed sequence of values. Values may be any element of [`JsonValue`].
#[derive(Debug, Clone)]
pub struct JsonArray {
    list: Vec<JsonValue>,
}

impl JsonArray {
    /// Create an empty [`JsonArray`].
    pub fn new() -> JsonArray {
        JsonArray { list: vec![] }
    }

    /// Parse `json` to [`JsonArray`].
    ///
    /// Return [`ErrorKind::TypeNotMatch`] if the parsed result
    /// is not a [`JsonValue::Array`].
    pub fn create(json: &str) -> Result<JsonArray> {
        let json_value = JsonTokener::new(json).next_value()?;
        if let JsonValue::Array(array) = json_value {
            Ok(array)
        } else {
            Err(Error::new(
                ErrorKind::TypeNotMatch,
                "Need JsonValue::Array but not.",
            ))
        }
    }

    /// Add a [`JsonValue`].
    pub fn push<T: Into<JsonValue>>(&mut self, value: T) {
        self.list.push(value.into());
    }

    /// Add a [`JsonValue`] on `index`.
    ///
    /// # Paincs
    ///
    /// Panics if `index > len`.
    pub fn insert<T: Into<JsonValue>>(&mut self, value: T, index: usize) {
        self.list.insert(index, value.into())
    }

    /// Get an immutable borrow of the value at `index` position
    ///
    /// Return [`None`] if `index` out of bounds.
    pub fn get(&self, index: usize) -> Option<&JsonValue> {
        self.list.get(index)
    }

    /// Get a mutable borrow of the value at `index` position
    ///
    /// Return [`None`] if `index` out of bounds.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut JsonValue> {
        self.list.get_mut(index)
    }

    /// Check if the value at the `index` position is [`JsonValue::Null`].
    pub fn is_null(&self, index: usize) -> bool {
        if let Some(JsonValue::Null) = self.get(index) {
            true
        } else {
            false
        }
    }

    /// Remove and return the value at `index`.
    ///
    /// Return [`None`] if `index` out of bounds.
    pub fn remove(&mut self, index: usize) -> Option<JsonValue> {
        if index < self.len() {
            Some(self.list.remove(index))
        } else {
            None
        }
    }

    /// Returns the number of values in this array.
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Check array is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl JsonBuilder for JsonArray {
    fn build(&self, mut json: String, pretty: bool, level: usize, indent: &str) -> String {
        json.push('[');

        let last = if self.is_empty() { 0 } else { self.len() - 1 };
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

// For print and convert to string.
impl Display for JsonArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build(String::new(), false, 0, ""))
    }
}

// For convert to pretty string.
impl ToJson for JsonArray {
    fn pretty(&self) -> String {
        self.to_json(true, "| ")
    }

    fn to_json(&self, pretty: bool, indent: &str) -> String {
        self.build(String::new(), pretty, 0, indent)
    }
}
