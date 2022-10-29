use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub msg: &'static str,
}

impl Error {
    pub fn new(kind: ErrorKind, msg: &'static str) -> Error {
        Error { kind, msg }
    }
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
            msg: "parse to int error"
        }
    }
}

impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Self {
        Error {
            kind: NumberParseError,
            msg: "parse to float error"
        }
    }
}
