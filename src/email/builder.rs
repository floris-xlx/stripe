//! ## Builder to create an email
//!
use crate::email::Email;

impl Email {
    /// # new
    /// Creates a new instance of the `Email` struct with specified details.
    ///
    /// ## Arguments
    /// - `to`: `String` - The recipient's email address.
    /// - `from`: `String` - The sender's email address.
    /// - `subject`: `String` - The subject line of the email.
    /// - `body`: `String` - The main content of the email.
    ///
    /// ## Returns
    /// Returns a new instance of `Email`.
    ///
    /// ## Examples
    /// ```rust
    /// let email = Email::new(
    ///     "recipient@example.com".to_string(),
    ///     "floris@xylex.ai".to_string(),
    ///     "Greetings".to_string(),
    ///     "Hello, how are you?".to_string(),
    /// );
    /// ```
    pub fn new(to: String, from: String, subject: String, body: String) -> Self {
        Self {
            to,
            from,
            subject,
            body,
        }
    }
}
