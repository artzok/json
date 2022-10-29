extern crate core;

mod tokener;
mod json_value;
mod json_object;
mod json_array;
mod error;

use std::result;

pub use json_array::JsonArray;
pub use json_value::JsonValue;
pub use  json_object::JsonObject;

pub use error::{Error, ErrorKind};

///
/// 全局解析方法，传入字符串，如果解析成功，则返回 `Ok(JsonValue)`
/// 如果解析失败，则返回 `Err(Error{kind: xxx})` 
/// 
pub fn parse(str: &str) -> Result<JsonValue> {
    return tokener::JsonTokener::new(str).next_value();
}

///
/// json 解析获取存取结果
/// 
pub type Result<T> = result::Result<T, Error>;

/// 内部使用：将 JsonValue 构建为 String 的方法
trait JsonBuilder {
    fn build(&self, json: String, pretty: bool, level: usize, indent: &str) -> String;
}

/// 公开用法：将 JsonValue 转为 String 类型 
pub trait ToJson {
    fn pretty(&self) ->String;
    fn to_json(&self, pretty: bool, indent: &str) -> String;
}