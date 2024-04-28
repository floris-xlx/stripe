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

/// ## Email Provider
/// This is the email provider that will be used to send out emails
///
/// ### Providers
/// - `resend` - This is a free email provider that lets you send up to 3k emails a month
/// - `smtp` - You can use any smtp provider you want like gmail, sendgrid, etc
///
/// ### Implementations
/// - [`from_str`](#from-string) - This will convert a string into an email provider
/// - [`to_string`](#to-string) - This will convert the email provider into a string
///
/// ### Usage example
///
///
pub enum EmailProvider {
    Resend,
    Smtp,
}

// temp clippy patches FIXME
#[allow(clippy::should_implement_trait)]
#[allow(clippy::inherent_to_string)]

impl EmailProvider {
    /// ## From String
    /// This will convert a string into an email provider
    ///
    /// ### Arguments
    /// - [`provider`](#providers) - This is the email provider as a string
    ///
    /// ### Returns
    /// This will return the email provider as an enum
    ///
    /// ### Example
    /// This will convert a string into an email provider
    /// ```rust
    /// let provider = EmailProvider::from_str("resend");
    /// ```
    ///
    pub fn from_str(provider: &str) -> Self {
        match provider {
            "resend" => EmailProvider::Resend,
            "smtp" => EmailProvider::Smtp,
            _ => EmailProvider::Resend,
        }
    }

    /// ## To string
    /// This will convert the email provider into a string
    ///
    /// ### Returns
    /// This will return the email provider as a string
    ///
    /// ### Example
    /// This will give you the email provider as a string
    /// ```rust
    /// let provider = EmailProvider::Resend;
    /// let provider_string = provider.to_string();
    ///
    /// assert_eq!(provider_string, "resend");
    /// ```
    pub fn to_string(&self) -> String {
        match self {
            EmailProvider::Resend => "resend".to_string(),
            EmailProvider::Smtp => "smtp".to_string(),
        }
    }
}

/// ## Email Configurations
/// This is the configuration for the email client
///
/// ### Fields
/// - `sender_email` - This is the email address that will be sending the email
/// - `sender_name` - This is the name of the sender_email
/// - [`provider`](#providers) - This is the email provider that will be used to send out emails
///
pub struct EmailConfig {
    pub sender_email: String,
    pub sender_name: String,
    pub provider: EmailProvider,
}
