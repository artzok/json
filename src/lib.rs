use std::{fmt::{Display, write}, result, io};

mod json_array;
mod json_field;
mod json_object;

///
/// JSON 元素类型：
/// * Object: {}
/// * Array: []
/// * Field: i32, bool, String ect.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementType {
    Object,
    Array,
    Field,
}

///
/// JSON 元素类型必须实现的 trait
///
/// 通过 get_type 获取类型枚举可以用来在运行时判断 `trait object` 所属类型
///
trait JSONElement {
    fn get_type(&self) -> ElementType;
}

///
/// JSON 解析错误
///
pub struct Error {
    pub kind: ErrorKind,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

///
/// JSON 解析错误类型
///
pub enum ErrorKind {
    NotFound,
    TypeError,
}

impl ErrorKind {
    pub fn as_str(&self) -> &'static str {
        return match self {
            Self::NotFound => "NotFound: Not found value of the key",
            Self::TypeError => "TypeError: the value of the key not match",
             _ => ""
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind.as_str())
        // TODO other info
    }
}

pub type Result<T> = result::Result<T, Error>;
