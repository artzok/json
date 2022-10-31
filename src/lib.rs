extern crate core;

mod error;
mod json_array;
mod json_object;
mod json_value;
mod tokener;
mod utils;

use std::result;

pub use error::{Error, ErrorKind};
pub use json_array::JsonArray;
pub use json_object::JsonObject;
pub use json_value::JsonValue;

/// Global convenience parsing method.
///
/// Return [`Ok(JsonValue)`] on success.
/// Parse failure returns [`Err(Error)`].
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
/// This type is used `json` package for any operation which may produce an error.
///
/// This typedef is generally used to avoid writing out [`Error`] directly and
/// is otherwise a direct mapping to [`Result`].
///
/// # See also:
///
/// [`parse`] method return type.
/// ```
pub type Result<T> = result::Result<T, Error>;

/// Used to build JSON strings from [`JsonValue`], for internal use only.
trait JsonBuilder {
    fn build(&self, json: String, pretty: bool, level: usize, indent: &str) -> String;
}

/// Public trait for converting [`JsonValue`] to JSON string.
///
/// All json elements already implement this trait. Including:
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
/// println!("{}", json.to_json(true, " "));
/// ```
///
pub trait ToJson {
    fn pretty(&self) -> String;
    fn to_json(&self, pretty: bool, indent: &str) -> String;
}