use std::fmt;
use std::os::raw::c_int;

#[derive(Debug)]
pub enum Error {
    Io,
    InvalidParam,
    Access,
    NoDevice,
    NotFound,
    Busy,
    Timeout,
    Overflow,
    Pipe,
    Interrupted,
    NoMem,
    NotSupported,
    NoValidEEPROMHeader,
    StringValueTooLong,
    StringDescriptorInvalid,
    StringDescriptorTooLong,
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<c_int> for Error {
    fn from(e: c_int) -> Self {
        match e {
            -1 => Error::Io,
            -2 => Error::InvalidParam,
            -3 => Error::Access,
            -4 => Error::NoDevice,
            -5 => Error::NotFound,
            -6 => Error::Busy,
            -7 => Error::Timeout,
            -8 => Error::Overflow,
            -9 => Error::Pipe,
            -10 => Error::Interrupted,
            -11 => Error::NoMem,
            -12 => Error::NotSupported,
            -13 => Error::NoValidEEPROMHeader,
            -14 => Error::StringValueTooLong,
            -15 => Error::StringDescriptorInvalid,
            -16 => Error::StringDescriptorTooLong,
            _ => Error::Unknown,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io => write!(f, "Input/output error"),
            Error::InvalidParam => write!(f, "Invalid parameter"),
            Error::Access => write!(f, "Access denied"),
            Error::NoDevice => write!(f, "No such device"),
            Error::NotFound => write!(f, "Not found"),
            Error::Busy => write!(f, "Resource busy"),
            Error::Timeout => write!(f, "Operation timed out"),
            Error::Overflow => write!(f, "Overflow"),
            _ => write!(f, "An unknown error occurred"),
        }
    }
}

impl std::error::Error for Error {}
