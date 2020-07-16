use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error;

use crate::failure;

#[derive(Debug)]
pub enum PhoneError {
    /// fail to parse
    Failure(failure::Error),
    /// parsed successfully, but is invalid according to the country
    Invalid,
}

impl From<failure::Error> for PhoneError {
    #[inline]
    fn from(error: failure::Error) -> Self {
        PhoneError::Failure(error)
    }
}

impl Display for PhoneError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            PhoneError::Failure(error) => Display::fmt(error, f),
            PhoneError::Invalid => f.write_str("invalid phone number"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for PhoneError {}
