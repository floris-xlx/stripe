//! ## Checks
//! 
//! ### Table of contents
//! - [Checking if email is valid]













/// ## Checking if emails are unique
/// 
/// Super simple function, this will prevent self-sending of emails
/// 
/// ### Usage example
/// ```
/// let email_1 = "test@test.com";
/// let email_2 = "test@test.com";
/// let is_unique = are_emails_unique(email_1, email_2);
/// assert!(is_unique);
/// ```
/// 
pub fn are_emails_unique(
    email_1: &str,
    email_2: &str,
) -> bool {

    email_1 != email_2
}

