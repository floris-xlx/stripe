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
    /// # verify_email
    /// Checks if the email address stored in this instance matches a standard email format using a regular expression.
    ///
    /// ## Arguments
    /// - `&self`: A reference to the instance of `EmailAddress` containing the email to verify.
    ///
    /// ## Returns
    /// Returns `true` if the email matches the regular expression, otherwise returns `false`.
    ///
    /// ## Example: Verifying an email address
    /// ```rust
    /// let email = EmailAddress { email: "example@example.com".to_string() };
    /// assert!(email.verify_email());
    /// ```
    pub fn verify_email(&self) -> bool {
        let email_regex = r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$";
        let re = Regex::new(email_regex).unwrap();
        re.is_match(&self.email)
    }


    /// # to_string
    /// Converts the `EmailAddress` instance into a `String` representing the email address.
    ///
    /// ## Arguments
    /// - `&self`: A reference to the `EmailAddress` instance.
    ///
    /// ## Returns
    /// Returns a `String` that contains the email address.
    ///
    /// ## Example: Converting an `EmailAddress` instance to a `String`
    /// ```rust
    /// let email_address = EmailAddress { email: "example@example.com".to_string() };
    /// assert_eq!(email_address.to_string(), "example@example.com");
    /// ```
    pub fn to_string(&self) -> String {
        self.email.clone()
    }
}
