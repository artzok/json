use std::{
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
};

///
/// Json parse or operations all possible error messages.
///
/// `kind` is [`ErrorKind`], indicates the type of error.
///
/// `msg` has a more detailed description.
///
///
#[derive(Debug, Clone, Copy)]
pub struct Error {
    pub kind: ErrorKind,
    pub msg: &'static str,
}

/// All type of [`Error`].
///
/// [`ErrorKind::EOF`] Expected more characters, but ended early.
///
/// [`ErrorKind::SyntaxError`] Get unexpected character.
///
/// [`ErrorKind::NumberParseError`] Integer or float parsing error.
///
/// [`CastError`] Type conversion error.
///
/// [`NotFound`] No value found for key.
/// 
/// [`TypeNotMatch`] Type mismatch.
/// 
/// [`ValueNull`] The value of key is null.
///
#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    EOF,
    SyntaxError,
    NumberParseError,
    CastError,
    NotFound,
    TypeNotMatch,
    ValueNull,
}

impl Error {
    pub fn new(kind: ErrorKind, msg: &'static str) -> Error {
        Error { kind, msg }
    }
}

use ErrorKind::*;

/// Create from [`ParseIntError`].
impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error {
            kind: NumberParseError,
            msg: "parse to int error",
        }
    }
}

/// Create from [`ParseFloatError`].
impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Self {
        Error {
            kind: NumberParseError,
            msg: "parse to float error",
        }
    }
}

/// Print or convert to string.
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.kind, f)?;
        write!(f, ":{}", self.msg)
    }
}

/// Print or convert to string.
impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}