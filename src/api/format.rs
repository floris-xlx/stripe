//! ## Formatting
//!
//! This module contains all the formatting that are used in the API.
//! It provides functionality to create a new API instance with formatted host, port, and address fields.
//!

use crate::api::Api;
use crate::ConfigSetup;

/// Implementation of the `Api` struct.
impl Api {
    /// Constructs a new `Api` instance.
    ///
    /// This function initializes an `Api` instance with host, port, and address fields.
    /// The host and port are fetched from the configuration setup, and the address is formatted as "host:port".
    ///
    /// # Returns
    /// A new `Api` instance with initialized fields.
    pub fn new() -> Self {
        Api {
            host: Self::host(),
            port: Self::port(),
            address: Self::address(),
        }
    }

    /// Fetches the host from the configuration setup.
    ///
    /// This function creates a new `ConfigSetup` instance and returns the host field.
    ///
    /// # Returns
    /// A `String` representing the host.
    fn host() -> String {
        let config: ConfigSetup = ConfigSetup::new();

        config.host
    }

    /// Fetches the port from the configuration setup.
    ///
    /// This function creates a new `ConfigSetup` instance and returns the port field.
    ///
    /// # Returns
    /// A `u64` representing the port.
    fn port() -> u64 {
        let config: ConfigSetup = ConfigSetup::new();

        config.port
    }

    /// Formats and returns the address as "host:port".
    ///
    /// This function creates a new `ConfigSetup` instance and formats the address using the host and port fields.
    ///
    /// # Returns
    /// A `String` representing the formatted address.
    fn address() -> String {
        let config: ConfigSetup = ConfigSetup::new();

        format!("{}:{}", config.host, config.port)
    }
}
