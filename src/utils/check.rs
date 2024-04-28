//! ## Checks
//! 
//! ### Table of contents
//! - [Checking if email is valid]













/// # are_emails_unique
/// Determines whether two email addresses are unique (not identical).
///
/// ## Arguments
/// - `email_1`: `&str` - The first email address to compare.
/// - `email_2`: `&str` - The second email address to compare.
///
/// ## Returns
/// Returns `true` if the two email addresses are not the same, otherwise returns `false`.
///
/// ## Examples
/// ```
/// let email_1 = "user1@example.com";
/// let email_2 = "user2@example.com";
/// assert!(are_emails_unique(email_1, email_2)); // This will pass because emails are unique
///
/// let email_1 = "user@example.com";
/// let email_2 = "user@example.com";
/// assert!(!are_emails_unique(email_1, email_2)); // This will pass because emails are not unique
/// ```
pub fn are_emails_unique(
    email_1: &str,
    email_2: &str,
) -> bool {
    email_1 != email_2
}
