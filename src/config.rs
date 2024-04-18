use std::error::Error;

use crate::Config;


use std::fs;
use serde_yaml;

impl Config {
    
    /// ## Create a new Config object
    /// 
    /// This function will create a new Config object the given config file path
    /// 
    /// ### Usage
    /// It will rely on the file path of the `stripe_discord.yaml` file, to offer more flexibility.
    /// You can supply the path to the file as a string yourself.
    /// 
    /// ```rust
    /// // Declaring a file called stripe_discord.yaml as config file in the root directory
    /// 
    /// let config = Config::new("stripe_discord.yaml").unwrap();
    /// ```
    /// 
    /// ### Arguments
    /// * `config_path` - The path to the config file
    /// 
    /// ### Returns
    /// A Result containing the Config object or a Boxed Error
    /// 
    /// ### Example
    /// ```rust
    /// let config = Config::new("stripe_discord.yaml").unwrap();
    /// ```
    /// 
    /// ### Errors
    /// 
    pub fn new(
        config_path: &str
    ) -> Result<Config, Box<dyn Error>> {
        
        let config_str = fs::read_to_string(config_path)?;
        let config: Config = serde_yaml::from_str(&config_str)?;

        Ok(config)
    }

    pub fn get_db_provider(&self) -> &str {
        &self.db_provider
    }

    pub fn get_email_provider(&self) -> &str {
        &self.email_provider
    }
}
