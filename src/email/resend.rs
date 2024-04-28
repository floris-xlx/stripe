//! ## Sending emails with `Resend`
//!
//! This module allows you to send emails with `Resend`. You can use this module by setting the `email.Provider` to `resend` in your `config.yaml` file.
//!
//! ### Requirements of `Resend`
//! - `RESEND_API_KEY` - Your Resend API key
//! - `RESEND_EMAIL` - The email address you want to send emails from
//! - `RESEND_NAME` - The name you want to send emails from
//! - `EMAIL_TEMPLATE_ID` - The template ID you want to use
//!
//!

use crate::Organization;
use resend_email_rs::{Attachment, MailHtml, ResendClient};

/// # authenticate
/// Creates a new `ResendClient` instance using the provided API key to authenticate with the Resend service.
///
/// ## Arguments
/// - `api_key`: A `String` containing the API key for authenticating with Resend.
///
/// ## Returns
/// Returns a `ResendClient` instance which can be used to interact with Resend services.
///
/// ## Examples
/// ```rust
/// let api_key = "your_resend_api_key".to_string();
/// let client = authenticate(api_key);
/// ```
pub fn authenticate(api_key: String) -> ResendClient {
    ResendClient::new(api_key)
}


/// # send_email_html
/// Sends an HTML formatted email using the Resend service.
///
/// ## Arguments
/// - `client`: `ResendClient` - The client used to send the email.
/// - `organization`: `Organization` - The organization from which the email is sent. Used to determine the sender's email address.
/// - `to`: `Vec<String>` - A list of recipient email addresses.
/// - `subject`: `String` - The subject line of the email.
/// - `html`: `String` - The HTML content of the email.
/// - `attachments`: `Option<Vec<Attachment>>` - Optional list of attachments to include in the email.
///
/// ## Returns
/// - `Result<String, String>`: Returns either the message ID of the sent email as `Ok(String)` or an error message as `Err(String)`.
///
/// ## Examples
/// ```rust
/// use resend_email_rs::{ResendClient, Attachment};
/// use crate::Organization;
///
/// async fn example_send() {
///     let client = ResendClient::new("api_key".to_string());
///     let organization = Organization {
///         name: "Example Org".to_string(),
///         sender_email: "noreply@example.org".to_string(),
///     };
///     let recipients = vec!["user@example.com".to_string()];
///     let subject = "Welcome!".to_string();
///     let html_content = "<h1>Hello, World!</h1>".to_string();
///     let attachments = None;
///
///     let result = send_email_html(client, organization, recipients, subject, html_content, attachments).await;
///     match result {
///         Ok(message_id) => println!("Email sent with ID: {}", message_id),
///         Err(e) => println!("Failed to send email: {}", e),
///     }
/// }
/// ```
pub async fn send_email_html(
    client: ResendClient,
    organization: Organization,
    to: Vec<String>,
    subject: String,
    html: String,
    attachments: Option<Vec<Attachment>>,
) -> Result<String, String> {
    let from = organization.sender_email;

    let mail = MailHtml {
        from,
        to,
        subject,
        html,
        attachments,
    };

    match client.send(&mail).await {
        Ok(email_sent_status) => Ok(email_sent_status.id),
        Err(e) => Err(e.to_string()),
    }
}
