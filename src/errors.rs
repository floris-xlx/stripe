//! ## Errors
//!
//! ### How to map an error type
//!
//!
//! This module contains all the errors that are used in the application
//!
//! ### Variants
//!
//!
//! ### Notes
//!
//!

use fmt::{Display, Formatter, Result};
use std::error::Error;
use std::fmt;

use crate::ConfigError;

impl Display for ConfigError {
    /// ## Formatting
    ///
    /// This function is used to format the error message
    ///
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            ConfigError::FileNotFound(ref path) => write!(f, "File not found at path: {}", path),
            ConfigError::InvalidFileType(ref path) => write!(
                f,
                "Invalid file type, expected .yaml file at path: {}",
                path
            ),
        }
    }
}

impl Error for ConfigError {}
