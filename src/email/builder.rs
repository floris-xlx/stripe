//! ## Builder to create an email
//!
use crate::email::Email;

impl Email {
    pub fn new(to: String, from: String, subject: String, body: String) -> Self {
        Self {
            to,
            from,
            subject,
            body,
        }
    }
}
