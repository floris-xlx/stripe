#![allow(clippy::inherent_to_string)]

//! ## Formatting email related data
//!
//! ### Table of contents
//! - Regex checking if a sender or recipient email CAN be a valid email address
//!
//! ### Return types
//!
//! #### Error returns
//!
//! #### Success returns
//!
//!
//! ### Notes
//! - This can be disabled by setting the `.env` variable to `ALLOW_DIRTY_EMAIL=1` (This is
//! disadvised)
//!
//!
//!

// importing regex
use regex::Regex;

// importing the email object, make sure to implement a to_string
use crate::email::EmailAddress;

impl EmailAddress {
    /// ## Check if email can be valid with Regex
    ///
    ///
    pub fn verify_email(&self) -> bool {
        let email_regex = r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$";
        let re = Regex::new(email_regex).unwrap();
        re.is_match(&self.email)
    }

    /// ## Implements `to_string`
    ///
    pub fn to_string(&self) -> String {
        self.email.clone()
    }
}
