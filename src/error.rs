use std::{
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
};

///
/// Json parse or operations fail info.
///
/// `kind` indication error kind.
///
/// `msg` has more specific description.
///
///
#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub msg: &'static str,
}

/// All kind of json parse or operations error.
///
/// [`ErrorKind::EOF`] An error returned when an operation could not be completed because an
/// "end of file" was reached prematurely.
///
/// [`ErrorKind::SyntaxError`] Get a not expect character.
///
/// [`ErrorKind::NumberParseError`] Parse integer or float number error.
///
/// [`CastError`] Except type and save type not match or parse escape character error.
///
/// [`NotFound`] Not found value of key.
/// 
/// [`TypeNotMatch`] Type mismatch.
///
#[derive(Debug)]
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

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error {
            kind: NumberParseError,
            msg: "parse to int error",
        }
    }
}

impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Self {
        Error {
            kind: NumberParseError,
            msg: "parse to float error",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.kind, f)?;
        write!(f, ":{}", self.msg)
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}
