extern crate core;

mod error;
mod json_array;
mod json_object;
mod json_value;
mod tokener;
mod utils;

use std::{borrow::Cow, result};

use colored::Colorize;
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
pub type Result<T> = result::Result<T, Error>;

pub struct BuildConfig<'a> {
    pretty: bool,                                                              // 漂亮格式化
    indent: &'a str,                                                           // 前导字符
    check_nest: bool,                                                          // 检查嵌套 json
    value_converter: Box<dyn for<'b> Fn(&JsonValue, &'b str) -> Cow<'b, str>>, // 值转换
    key_converter: Box<dyn Fn(&str) -> Cow<str>>,                              // 键转换
    control_converter: Box<dyn Fn(char) -> String>,                            // 控制字符转换
}

fn default_value_convert<'a>(_: &JsonValue, text: &'a str) -> Cow<'a, str> {
    Cow::from(text)
}

fn default_key_convert(key: &str) -> Cow<str> {
    Cow::from(key)
}

fn default_control_convert(ctl: char) -> String {
    ctl.to_string()
}

fn pretty_value_convert<'a>(json: &JsonValue, text: &'a str) -> Cow<'a, str> {
    match json {
        JsonValue::Null | JsonValue::Bool(_) => Cow::from(format!("{}", text.black().bold())),
        JsonValue::Int(_) | JsonValue::Uint(_) | JsonValue::Float(_) => {
            Cow::from(format!("{}", text.cyan().bold()))
        }
        JsonValue::String(_) => Cow::from(format!("{}", text.red())),
        _ => Cow::from(text),
    }
}

fn pretty_key_convert(key: &str) -> Cow<str> {
    Cow::from(format!("{}", key.green().bold()))
}

fn pretty_control_convert(ctl: char) -> String {
    let colored = match ctl {
        '[' | ']' | '{' | '}' => ctl.to_string().red().bold(),
        '"' | ',' | ':' => ctl.to_string().truecolor(128, 128, 128).bold(),
        _ => ctl.to_string().black(),
    };
    format!("{}", colored)
}

impl<'a> BuildConfig<'a> {
    pub fn new(pretty: bool, indent: &'a str, check_nest: bool, colored: bool) -> BuildConfig {
        BuildConfig {
            pretty,
            indent,
            check_nest,
            value_converter: Box::new(if colored {
                pretty_value_convert
            } else {
                default_value_convert
            }),
            key_converter: Box::new(if colored {
                pretty_key_convert
            } else {
                default_key_convert
            }),
            control_converter: Box::new(if colored {
                pretty_control_convert
            } else {
                default_control_convert
            }),
        }
    }
    fn default() -> BuildConfig<'static> {
        BuildConfig {
            pretty: false,
            indent: "",
            check_nest: false,
            value_converter: Box::new(default_value_convert),
            key_converter: Box::new(default_key_convert),
            control_converter: Box::new(default_control_convert),
        }
    }

    fn pretty() -> BuildConfig<'static> {
        BuildConfig {
            pretty: true,
            indent: "| ",
            check_nest: false,
            value_converter: Box::new(pretty_value_convert),
            key_converter: Box::new(pretty_key_convert),
            control_converter: Box::new(pretty_control_convert),
        }
    }
}

/// Used to build JSON strings from [`JsonValue`], for internal use only.
trait JsonBuilder {
    fn build(&self, json: String, level: usize, cfg: &BuildConfig) -> String;
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
/// use json::BuildConfig;
///
/// let str = "{\"key\" : \"value\", \"array\": [1, \"rust\", false, 12.5]}";
/// let json = json::parse(str).unwrap();
/// println!("{}", json.pretty());
/// println!("{}", json.to_json(&BuildConfig::new(true, "| ", false)));
/// ```
///
pub trait ToJson {
    fn pretty(&self) -> String;
    fn to_json(&self, cfg: &BuildConfig) -> String;
}
