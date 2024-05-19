#![allow(rustdoc::bare_urls)]

//! # Stripe with Discord Automation Integration
//!
//! This project integrates Stripe with Discord automation. It uses a Rust SDK to listen to Stripe webhooks and then assigns roles in Discord and sends emails via a chosen email provider.
//!
//! ## Getting started
//! To get started, add the following dependencies to your `Cargo.toml` file:
//! ```toml
//! [dependencies]
//! stripe_discord = "0.1.0"
//! ```
//!
//! ## Features
//! - Stripe webhook listener
//! - Discord role assignment
//! - Email notifications
//! - Supabase and Sled database supported
//!
//! ## Prerequisites
//! - Stripe account with developer access
//! - Discord bot with OAuth2 access
//! - Email provider (Resend or SMTP)
//! - Supabase account (Free)
//! - `SUPABASE_URL` and `SUPABASE_KEY` environment variables
//! - `STRIPE_WEBHOOK_SECRET`, `STRIPE_PRIVATE_API_KEY`, and `STRIPE_PUBLISH_KEY` environment variables
//! - `SMTP_HOST`, `SMTP_PORT`, and `SENDER_EMAIL` environment variables (only if using SMTP)
//! - `RESEND_API_KEY` and `SENDER_EMAIL` environment variables (only if using Resend)
//!
//!
//! ## Overwriting the default Supabase table names and column names
//! You can overwrite the default Supabase table names and column names by setting the following environment variables:
//! - `OVERWRITE_STRIPE_CUSTOMER_TABLE_NAME` (default: `stripe_customer_data`) to overwrite the default table name for the customer data in Supabase
//!
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
//!
//! ## Automatic Emails
//! When automatic emails are enabled, you can choose between Resend or SMTP. Templates go in HTML format in `./email/templates`.
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
//!
//! ## Databasing
//! In the `stripe_discord.yaml` file, you can choose between Sled and Supabase.
//!
//! Supported databasing instances are:
//! - `sled` (local)
//! - `supabase` (online)
//!
//!
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
pub mod data;
pub mod db;
pub mod discord;
pub mod email;
pub mod errors;
pub mod events;
pub mod log;
pub mod organization;
pub mod overwrite;
pub mod tests;
pub mod utils;
pub mod background;

use crate::email::EmailAddress;


/// ## Configuration #[derive(Debug)]
/// This will set the config for the `email` and for the `databasing` solutions
/// NOTE: THIS WILL BE USED TO EITHER REPLACE THE .YAML OR GENERATE ONE !!
#[derive(Debug)]
pub struct ConfigSetup {
    pub db_provider: String,
    pub email_provider: String,
    pub sender_email: String,
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


pub struct EmailConfig {
    pub sender_email: String,
    pub subject: String,
    pub template_url: String
}

impl EmailConfig {
    /// # new EmailConfig
    /// This function creates a new `EmailConfig` struct from the provided email, subject, and template url
    /// 
    /// ## Arguments
    /// - `sender_email` - The email address of the sender
    /// - `subject` - The subject of the email
    /// - `template_url` - The URL of the email template
    /// 
    /// ## Returns
    /// A new `EmailConfig` struct
    pub fn new(
        sender_email: String,
        subject: String,
        template_url: String
    )-> Self {

        Self {
            sender_email,
            subject,
            template_url
        }
    }
}


/// ## Organization struct
/// This struct represents the organization data that is used to create a new organization
/// profile
///
/// ### Fields
/// - `name` - The name of the organization
/// - `stripe_key` - The stripe key of the organization
/// - `stripe_secret` - The stripe secret of the organization
/// - `stripe_webhook_secret` - The stripe webhook secret of the organization
/// - `config` - The stripe endpoint config of the organization`
///
pub struct Organization {
    /// `The name of the organization that is used to identify the organization in the db`
    pub name: String,
    pub email_config: EmailConfig,
}


/// ## Customer ID for Stripe
/// This struct represents the customer ID for Stripe
///
/// ### Fields
/// - `id` - The customer id
///
/// ### Implementations
/// - `new` - This function creates a new `CustomerId` from the provided customer id
/// - `attach_email` - This function attaches an email to the customer id in Supabase
/// - `FetchEmail` - This trait is used to fetch the email of the customer id
///
#[derive(Debug, Clone)]
pub struct CustomerId {
    /// The customer id
    pub id: String,
}



impl CustomerId {
    /// # as_str
    /// Returns a string slice of the CustomerId's `id`.
    ///
    /// ## Arguments
    /// - `&self` - A reference to the instance of `CustomerId` from which the `id` is retrieved.
    ///
    /// ## Returns
    /// A string slice (`&str`) representing the `id` of the customer.
    ///
    /// ## Example: Getting the customer id as a string slice
    /// ```rust
    /// use stripe_discord::CustomerId;
    ///
    /// let customer_id = CustomerId::new("cus_12345".to_string());
    /// assert_eq!(customer_id.as_str(), "cus_12345");
    /// ```
    pub fn as_str(&self) -> &str {

        // return the id as a string slice
        self.id.as_str()
    }

}
