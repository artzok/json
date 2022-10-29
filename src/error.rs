use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    EOF, // 解析未成功时遇到结束
    SyntaxError,
    NumberParseError,
    CastError, // 类型转换错误
}

use ErrorKind::*;

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error {
            kind: NumberParseError,
        }
    }
}

impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Self {
        Error {
            kind: NumberParseError,
        }
    }
}
