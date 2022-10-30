extern crate core;

mod error;
mod json_array;
mod json_object;
mod json_value;
mod tokener;

use std::result;

pub use error::{Error, ErrorKind};
pub use json_array::JsonArray;
pub use json_object::JsonObject;
pub use json_value::JsonValue;

/// Global parse method for convenient.
///
/// Return `Ok(JsonValue)` if success.
/// Return `Err(Error)` if parse failed.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let str = "{\"key\":\"value\"}";
/// let json = json::parse(str).unwrap();
/// assert_eq!(json.to_string(), str);
/// ```
///
pub fn parse(str: &str) -> Result<JsonValue> {
    return tokener::JsonTokener::new(str).next_value();
}

/// A specialized [`Result`] type for JSON operations.
///
/// This type is broadly used across `json` package for any operation which may
/// produce an error.
///
/// This typedef is generally used to avoid writing out [`Error`] directly and
/// is otherwise a direct mapping to [`Result`].
///
/// # See also:
///
/// [`parse`] method return type.
/// ```
pub type Result<T> = result::Result<T, Error>;

/// use to build JSON string from [`JsonValue`], for internal use only.
trait JsonBuilder {
    fn build(&self, json: String, pretty: bool, level: usize, indent: &str) -> String;
}

/// Public trait for convert [`JsonValue`] to JSON string.
///
/// All json element must implement this trait. Includes:
///
/// 1. [`JsonValue`]
/// 2. [`JsonObject`]
/// 3. [`JsonArray`]
///
/// # Examples:
///
/// ```
/// use json::ToJson;
///
/// let str = "{\"key\" : \"value\", \"array\": [1, \"rust\", false, 12.5]}";
/// let json = json::parse(str).unwrap();
/// println!("{}", json.pretty());
/// ```
///
pub trait ToJson {
    fn pretty(&self) -> String;
    fn to_json(&self, pretty: bool, indent: &str) -> String;
}
