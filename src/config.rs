//! ## Configuration settings
//!
//!
//! ### Table of contents
//!
//! ### Errors
//!
//!
//! ### Troubleshooting
//!
//!
//! ### Usage example

#![allow(unused_imports)]

use serde_json::Value;
use serde_yaml;
use std::{error::Error, fs, fs::File, io::BufReader};

use crate::ConfigSetup;

impl Default for ConfigSetup {
    /// # default
    /// Creates a default `ConfigSetup` instance with predefined values.
    ///
    /// ## Arguments
    /// This function takes no arguments.
    ///
    /// ## Returns
    /// Returns a `ConfigSetup` instance with the following predefined values:
    /// - `db_provider`: "supabase" - Default database provider.
    /// - `email_provider`: "resend" - Default email provider.
    /// - `sender_email`: "test@example.com" - Default sender email address.
    /// - `host`: "0.0.0.0" - Default host address.
    /// - `port`: 8080 - Default port number.
    /// - `supabase_url`: "https://xxx.supabase.co" - Default Supabase URL.
    /// - `supabase_key`: "xxx" - Default Supabase API key.
    ///
    /// ## Examples
    /// ```rust
    /// let config = ConfigSetup::default();
    /// assert_eq!(config.db_provider, "supabase");
    /// assert_eq!(config.email_provider, "resend");
    /// assert_eq!(config.sender_email, "test@example.com");
    /// assert_eq!(config.host, "0.0.0.0");
    /// assert_eq!(config.port, 8080);
    /// assert_eq!(config.supabase_url, "https://xxx.supabase.co");
    /// assert_eq!(config.supabase_key, "xxx");
    /// ```
    fn default() -> Self {
        ConfigSetup {
            db_provider: "supabase".to_string(),
            email_provider: "resend".to_string(),
            sender_email: "test@example.com".to_string(),
            host: "0.0.0.0".to_string(),
            port: 8080,
            supabase_url: "https://xxx.supabase.co".to_string(),
            supabase_key: "xxx".to_string(),
        }
    }
}

impl ConfigSetup {
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
    pub fn new() -> Self {
        let mut config: ConfigSetup = ConfigSetup {
            db_provider: String::new(),
            email_provider: String::new(),
            sender_email: String::new(),
            host: String::new(),
            port: 0,
            supabase_url: String::new(),
            supabase_key: String::new(),
        };

        config.load();

        config
    }

    fn load(&mut self) {
        // open and read our .yaml file
        let file: File =
            File::open("stripe_discord.yaml").expect("Failed to open the stripe_discord.yaml file");
        let reader: BufReader<File> = BufReader::new(file);

        // read and iterate over the value keys
        let value: Value =
            serde_yaml::from_reader(reader).expect("Failed to read stripe_discord.yaml");

        // load our pre-configed yaml objects from the reader
        self.db_provider = value["Db"]["Provider"]
            .as_str()
            .unwrap_or("supabase")
            .to_string();
        self.email_provider = value["Email"]["Provider"]
            .as_str()
            .unwrap_or("resend")
            .to_string();
        self.sender_email = value["Email"]["Sender"]
            .as_str()
            .unwrap_or("floris@xylex.ai")
            .to_string();
        self.host = value["Api"]["Host"]
            .as_str()
            .unwrap_or("0.0.0.0")
            .to_string();
        self.port = value["Api"]["Port"].as_u64().unwrap_or(8080);
        self.supabase_url = value["Db"]["SupabaseUrl"]
            .as_str()
            .unwrap_or("https://xxx.supabase.co")
            .to_string();

        // load env vars
        self.supabase_key = std::env::var("SUPABASE_KEY")
            .unwrap_or("xxx".to_string())
            .to_string();
    }
}

