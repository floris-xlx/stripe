#![allow(rustdoc::bare_urls)]

//! # Stripe with Discord Automation Integration
//!
//! This project integrates Stripe with Discord automation. It uses a Rust SDK to listen to Stripe webhooks and then assigns roles in Discord and sends emails via a chosen email provider.
//!
//!
//! REMINDER TO HANDLE ENV VARIABLE:
//! ALLOW_DIRTY_EMAIL=1
//!
//!
//! ## Required Environment Variables
//! - `STRIPE_WEBHOOK_SECRET`
//! - `STRIPE_PRIVATE_API_KEY`
//! - `STRIPE_PUBLISH_KEY`
//! - `SUPABASE_URL`
//! - `SUPABASE_KEY`
//! - `RESEND_API_KEY`
//! - `SMTP_HOST`
//! - `SMTP_PORT`
//! - `SMTP_EMAIL_ADDRESS`
//! - `AWS_ACCESS_KEY_ID`
//! - `AWS_SECRET_ACCESS_KEY`
//! - `AWS_EMAIL`
//!
//! ## Automatic Emails
//! When automatic emails are enabled, you can choose between Resend, Amazon Simple Email, or SMTP. Templates go in HTML format in `./email/templates`.
//!
//! ### Dynamically populating emails
//! You can use these pre-built placeholders that are extracted from the Stripe payment to customize and design your email template around these with no additional effort.
//!
//! For placeholders:
//! - First name: `{{FirstName}}`
//! - Email: `{{Email}}`
//! - Full name: `{{FullName}}`
//! - Payment amount: `{{PaymentAmount}}`
//! - Purchase product name: `{{ProductName}}`
//! - Payment date: `{{PaymentDate}}`
//! These are used to personalize emails and use payment-oriented references.
//!
//! ### Picking an email provider
//! In the `stripe_discord.yaml` file, you can opt for one of the following email providers:
//! - `resend`
//! - `smtp`
//! - `ses` (Amazon Simple Email)
//!
//! ### Resend (Email option 1)
//! Resend is a free email provider that allows you to send 3k emails per month for free. You can sign up for an account [here](https://resend.io/).
//!
//! Required environment variables for Resend:
//! - `resend_api_key`
//! - `resend_email`
//!
//! Making `resend` your chosen email provider:
//! ```yaml
//! email:
//!   Provider: resend
//! ```
//!
//! Setting the correct environment variables for Resend:
//! ```env
//! RESEND_API_KEY=
//! RESEND_EMAIL=
//! ```
//!
//! ### SMTP (Email option 2)
//! SMTP is a protocol that allows you to send emails from virtually any supported SMTP provider, You will need to provide the following environment variables for SMTP:
//! - `smtp_host`
//! - `smtp_port`
//! - `smtp_email_address`
//!
//! Making `smtp` your chosen email provider:
//! ```yaml
//! email:
//!   Provider: smtp
//! ```
//!
//! Setting the correct environment variables for SMTP:
//! ```env
//! SMTP_HOST=
//! SMTP_PORT=
//! SMTP_EMAIL_ADDRESS=
//! ```
//!
//! ### SES, Amazon Simple Email (Email option 3)
//! SES is a paid email provider that allows you to send emails from virtually any supported SMTP provider, You will need to provide the following environment variables for SMTP:
//! - `smtp_host`
//! - `smtp_port`
//! - `smtp_email_address`
//!
//! Making `ses` your chosen email provider:
//! ```yaml
//! email:
//!   Provider: ses
//! ```
//!
//! Setting the correct environment variables for SES:
//! ```env
//! AWS_ACCESS_KEY_ID=
//! AWS_SECRET_ACCESS_KEY=
//! AWS_EMAIL=
//! ```
//!
//! ## Databasing
//! In the `stripe_discord.yaml` file, you can choose between Sled and Supabase.
//!
//! Supported databasing instances are:
//! - `sled` (local)
//! - `supabase` (online)
//!
//!
//! ### Sled (Db option 1)
//! Sled's local so there's no need for any keys or env variables, the downside is that you will need some type of disk retention to keep the database alive.
//!
//! Making `sled` your chosen provider:
//! ```yaml
//! db:
//!   Provider: sled
//! ```
//!
//! ### Supabase (Db option 2)
//! Supabase's online so you worry about less with a tiny bit of added latency and they have a very generous free tier
//!
//! Making `supabase` your chosen provider:
//! ```yaml
//! db:
//!   Provider: supabase
//! ```
//! If you do not have a supabase account/database set-up you can make a free account and use their free tier here: https://supabase.io/
//!
//! If you need a Rust Supabase SDK, you can install my crate [here](https://crates.io/crates/supabase_rs)
//!
//! Setting the correct environment variables for Supabase:
//! - `supabase_url`
//! - `supabase_key`
//!
//! ```env
//! SUPABASE_URL=
//! SUPABASE_KEY=
//! ```
//!
//! ### Tests
//! You can run tests with `cargo test` to check if your configuration is correct.
//!
//! ## CLI
//! There is a CLI to add more organizations to your Stripe config.

// externally exposing the `regex` crate
extern crate regex;

pub mod api;
pub mod auth;
pub mod config;
pub mod db;
pub mod discord;
pub mod email;
pub mod errors;
pub mod organization;
pub mod tests;
pub mod utils;

use serde_derive::{Deserialize, Serialize};

use crate::email::EmailAddress;

/// ## Configuration #[derive(Debug)]
/// This will set the config for the `email` and for the `databasing` solutions
///
#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigSetup {
    pub db_provider: String,
    pub email_provider: String,
    pub sender_email: EmailAddress,
    pub host: String,
    pub port: u64,
    pub supabase_url: String,
    pub supabase_key: String,
}

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
/// use crate::ConfigError;
///
/// let error = ConfigError::FileNotFound("stripe_discord.yaml".to_string());
///
/// println!("{}", error);
/// ```
///
/// ## Usage
/// This enum is used to represent the different types of errors that can occur when working with the Config struct
///
#[derive(Debug, Clone)]
pub enum ConfigError {
    FileNotFound(String),
    InvalidFileType(String),
}

/// ## EndpointConfigStripe for `rocket`
/// This is your entry endpoint configuration that you will attach to your `endpoint_stripe` to
/// irrigate it with it's configuration.
///
///
/// ### Usage example
/// ```rust
///
///
/// ```
///
/// ### Arguments
/// - [`endpoint_route`] This will set the api route your endpoint will listen to, (STRIPE HAS TO
/// MATCH TO WHAT YOU SET HERE).
/// - [`sender_email`] The email that will send out for this stripe instance.
/// - [`stripe_publish_key`] This is the *LIVE* publishable key found in your stripe dashboard, for
/// more infro go to #FIXME
/// - [`stripe_webhook_secret`] This is the *LIVE* webhook secret that stripe will give you after
/// assigning an endpoint route in the Stripe dashboard
/// - [`stripe_private_key`] This is the *LIVE* private api key stripe will give you.
/// - [`email_template_path`] This has to lead to either HTTP or FilePath of what `.html` email
/// template should be sent out under the `sender_email`
/// - [`discord_client_id`] This is the discord `client_id` that is used for `Oath2` Configs
/// - [`discord_application_id`] This is the discord application id that is used to assign a
/// specific discord application
/// - [`discord_role_id`] This is the `role_id` members should receive or be revoked based on
/// Stripe dictation
/// - [`discord_guild_id`] This is the `guild_id` of your server where the members should receive
/// said `role_id`
/// - [`discord_bot_token`] This is the discord `bot_token` for authenticating into your `discord`
/// bot to mitigate `Oath2` limitations such as revoking roles when subscription fails, Read more
/// -> FIXME
/// - [`replace_keys_with_env_names`] When `enabled` it will extract the aforementioned from an
/// `.env` file by the by your provided `.env` names
///
///
/// ### Implementations
///
///
/// ### Returns
///
/// ### Errors
///
/// ### Notes
/// * Discord roles can only be revoked OUTSIDE of the traditional `Oath2` portal otherwise discord
/// users would need to supply permissions themselves
/// * When `replace_keys_with_env_names` - This DEFAULTS to FALSE, is enabled it will NOT accept the traditional keys,
///
/// FIXME
/// NONE OF THESE ARGUMENTS ARE `TYPED` NOR IMPLEMENTED OR HANDLED YET
///
/// MAKE THE ACTUAL ENDPOINT IMPLEMENTATION DERIVE FROM EITHER AN ORGANIZATION OR FROM A CONFIG
///
///
///
#[derive(Clone, Debug)]
pub struct EndpointConfigStripe {
    pub endpoint_route: String,
    // import the email type from the email module but make sure to implement a string derivative
    pub sender_email: EmailAddress,

    // handled by auth
    pub stripe_publish_key: String,
    pub stripe_webhook_secret: String,
    pub stripe_private_key: String,

    // email html file
    pub email_template_path: String,

    // handled by discord auth client router
    //
    //
    pub discord_client_id: String,
    pub discord_application_id: String,
    pub discord_role_id: i64,
    pub discord_guild_id: i64,
    pub discord_bot_token: String,
    pub replace_keys_with_env_names: bool,
}
