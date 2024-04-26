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

/// ## Authenticate with (Resend)[https://resend.io]
pub fn authenticate(api_key: String) -> ResendClient {
    ResendClient::new(api_key)
}

/// ## Send an email with Resend using HTML
///
/// This function sends an email using Resend with HTML content
///
/// ### Parameters
/// - `client` - The Resend client
/// - `to` - The email address you want to send the email to
/// - `subject` - The subject of the email
/// - `html` - The HTML content of the email
/// - `from` - The email address you want to send the email from
/// - `name` - The name you want to send the email from
///
/// ### Returns
/// - `Result<String, String>` - A result containing the message ID or an error message
///
pub async fn send_email_html(
    client: ResendClient,
    organization: Organization,
    to: Vec<String>,
    subject: String,
    html: String,
    attachments: Option<Vec<Attachment>>,
) -> Result<String, String> {
    // build the mail object

    // this will build the sender email based on the Organization irrigated
    let from = organization.sender_email;

    let mail = MailHtml {
        from,
        to,
        subject,
        html,
        attachments,
    };

    let email_sent_status = client.send(&mail).await.unwrap();

    Ok(email_sent_status.id)
}
