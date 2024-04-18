use std::fmt;
use std::error::Error;
use fmt::{
    Formatter, 
    Result, 
    Display
};

use crate::ConfigError;




impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            ConfigError::FileNotFound(ref path) => write!(f, "File not found at path: {}", path),
            ConfigError::InvalidFileType(ref path) => write!(f, "Invalid file type, expected .yaml file at path: {}", path),
        }
    }
}

impl Error for ConfigError {}
