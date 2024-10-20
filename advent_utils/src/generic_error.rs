use std::{error::Error, fmt::Display};
use core::num::ParseIntError;

#[derive(Debug)]
pub struct GenericError {
    message: String
}

impl GenericError {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn as_err<T>(message: String) -> Result<T, Self> {
        Err(Self::new(message))
    }
}

impl Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl From<&str> for GenericError {
    fn from(message: &str) -> Self {
        Self {
            message: String::from(message)
        }
    }
}

impl From<ParseIntError> for GenericError {
    fn from(other_error: ParseIntError) -> Self {
        Self {
            message: other_error.to_string()
        }
    }
}