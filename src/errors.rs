use std::fmt;
use std::error::Error;
use fmt::{
    Formatter, 
    Result, 
    Display
};

/// # ConfigError
/// 
/// This enum represents the different types of errors that can occur when working with the Config struct
/// 
/// ## Variants
/// 
/// - `FileNotFound` - Indicates that the file was not found at the specified path
/// - `InvalidFileType` - Indicates that the file type is not supported, expected .yaml file
/// 
/// ## Example
/// ```rust
/// use stripe_discord::ConfigError;
/// 
/// let error = ConfigError::FileNotFound("stripe_discord.yaml".to_string());
/// 
/// println!("{}", error);
/// ```
/// 
/// ## Usage
/// This enum is used to represent the different types of errors that can occur when working with the Config struct
/// 
#[derive(Debug)]
pub enum ConfigError {
    FileNotFound(String),
    InvalidFileType(String),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            ConfigError::FileNotFound(ref path) => write!(f, "File not found at path: {}", path),
            ConfigError::InvalidFileType(ref path) => write!(f, "Invalid file type, expected .yaml file at path: {}", path),
        }
    }
}

impl Error for ConfigError {}
