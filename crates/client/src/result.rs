//! Result of a the execution of a 4LW command.

use std::{io, num, result, str};

/// Possible errors returned by executing `ZK4LWCommand`s
#[derive(Debug, Fail)]
pub enum ZK4LWError {
    #[fail(display = "Failed to parse integer: {}", _0)]
    ParseIntError(#[cause] num::ParseIntError),

    #[fail(display = "Failed to parse string: {}", _0)]
    ParseStringError(String),

    #[fail(display = "Field missing from response: {}", _0)]
    MissingFieldError(&'static str),

    #[fail(display = "Encountered I/O error: {}", _0)]
    IoError(#[cause] io::Error),

    #[fail(display = "Response wasn't valid UTF-8: {}", _0)]
    Utf8Error(#[cause] str::Utf8Error),
}

impl From<num::ParseIntError> for ZK4LWError {
    fn from(val: num::ParseIntError) -> Self {
        Self::ParseIntError(val)
    }
}

impl From<io::Error> for ZK4LWError {
    fn from(val: io::Error) -> Self {
        Self::IoError(val)
    }
}

impl From<str::Utf8Error> for ZK4LWError {
    fn from(val: str::Utf8Error) -> Self {
        Self::Utf8Error(val)
    }
}

/// Result produced by the execution of a `ZK4LWCommand` using the `ZK4LWClient`
pub type ZK4LWResult<T> = result::Result<T, ZK4LWError>;

#[macro_export]
macro_rules! unwrap_or_error {
    ($($name:ident)*) => {
        $(
            match $name {
                Some(v) => v,
                None => return Err(ZK4LWError::MissingFieldError(stringify!($name))),
            }
        )*
    }
}