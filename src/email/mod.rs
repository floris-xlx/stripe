//! ## Emailing, This will handle all the email providers and send out emails
//!
//! All of these email providers are free (unless you pay for your smtp)
//!
//! `resend` lets you send up to 3k emails a month for free so make a free account and by the time
//! you push out 3k stripe emails you should be able to afford their 20$ plan :P
//!
//! ### Providers
//! - `resend`
//! - `smtp`
//!
//! ### Authenticating
//!
//!
//! ### Table of contents
//! - `client`
//! - `resend`
//! - `smtp`
//! - `templates`
//! - `builder`
//!
//!
//! ### Errors
//!
//!
//! ### Notes
//! Neither Smtp or Resend is currently implemented
//!
pub mod builder;
pub mod client;
pub mod resend;
pub mod smtp;
pub mod templates;
pub mod utils;

/// ## Email
///
///
///
#[derive(Debug, Clone)]
pub struct Email {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub body: String,
}

/// ## Bare bone `EmailAddress` type
///
/// ### Implementations
#[derive(Debug, Clone)]
pub struct EmailAddress {
    pub email: String,
}

/// ## Email implementations
/// Formats and verifies email addresses
///
///
/// ### implementations
/// - `to_string` - formats the email into type `String`
///
///
/// ### Usage example
/// ```rust
///
///
/// ```
///
/// ### Arguments
///
/// ### Returns
///
/// ### Errors
///
/// ### Notes
///
///
impl Email {}
