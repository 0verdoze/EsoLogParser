
use std::num::{ParseIntError, ParseFloatError};

use serde::{de, ser};

#[derive(Debug, derive_more::Display)]
pub enum Error {
    InvalidToken,
    UnexpectedEnd,
    ParseIntError,
    ParseFloatError,
    NotAList,
    ReaderNotExhausted,
    CustomError(String),
    UnsupportedOperation(&'static str),
}

impl ser::StdError for Error {}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::CustomError(format!("{}", msg))
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Self::ParseIntError
    }
}

impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Self {
        Self::ParseFloatError
    }
}

impl From<fast_float::Error> for Error {
    fn from(_: fast_float::Error) -> Self {
        Self::ParseFloatError
    }
}

#[derive(Debug, derive_more::Display)]
pub enum SerializeError {
    CustomError(String),
    UnsupportedOperation(&'static str),
}

impl ser::StdError for SerializeError { }

impl ser::Error for SerializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::CustomError(format!("{}", msg))
    }
}
