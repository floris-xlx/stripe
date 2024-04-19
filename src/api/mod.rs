//! ## API endpoints to expose for Stripe
//!
//! ### Table of contents


pub mod client;
pub mod errors;
pub mod events;
pub mod success;
pub mod format;

/// ## Base construction for the `Api`
///
/// ### Fields
/// - `host` - This is the host port that the api is exposed under
/// - `port` - This is the port that the the api is exposed under
///
/// ## Usage example
/// ```
/// let api = Api::new("localhost", 8080);
/// ```
#[derive(Debug, Clone)]
pub struct Api {
    pub host: String,
    pub port: u64,
    pub address: String
}




