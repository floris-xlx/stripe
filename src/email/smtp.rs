//! ## Mailing over an `SMTP` server
//!
//! This module allows you to send emails over an `SMTP` server. You can use this module by setting the `email.Provider` to `smtp` in your `config.yaml` file.
//!
//! ### Requirements of the `SMTP` server
//! - `smtp_host` - The host of the `SMTP` server (the ip or domain)
//! - `smtp_port` - The port of the `SMTP` server (usually 25, 465, or 587, your email provider will have this information)
