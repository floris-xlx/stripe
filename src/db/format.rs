//! ## Formatting database related data
//!
//! This module contains the format logic for the Database.
use crate::db::{SledDb, SupabaseDb};
use crate::ConfigSetup;
use supabase_rs::SupabaseClient;

/// Represents a connection to a Supabase database.
impl SupabaseDb {
    /// Constructs a new `SupabaseDb` instance.
    ///
    /// # Returns
    /// A new `SupabaseDb` instance with fields populated from the configuration.
    pub fn new() -> Self {
        Self {
            supabase_url: Self::supabase_url(),
            supabase_key: Self::supabase_key(),
            supabase: Self::supabase(),
        }
    }

    /// Retrieves the Supabase URL from the configuration.
    ///
    /// # Returns
    /// A `String` containing the Supabase URL.
    fn supabase_url() -> String {
        let config: ConfigSetup = ConfigSetup::new();

        config.supabase_url
    }

    /// Retrieves the Supabase Key from the configuration.
    ///
    /// # Returns
    /// A `String` containing the Supabase Key.
    fn supabase_key() -> String {
        let config: ConfigSetup = ConfigSetup::new();

        config.supabase_key
    }

    /// Creates a new Supabase client using the URL and Key.
    ///
    /// # Returns
    /// A `SupabaseClient` instance for interacting with the Supabase database.
    fn supabase() -> SupabaseClient {
        SupabaseClient::new(Self::supabase_url(), Self::supabase_key())
    }
}
