use stripe_discord::email::resend::authenticate;
use stripe_discord::email::resend::send_email_html;

use stripe_discord::Organization;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("Hello, world!");

    // create a new Organization
    let organization = Organization::new("cooking".to_string(), "billing@xylex.cloud".to_string());

    let resend_api_key = "xxx".to_string();

    // authenticate with resend
    let client = authenticate(resend_api_key);

    // send an email
    let to = vec!["floris@xylex.ai".to_string()];
    let subject = "Welcome to Cooking".to_string();
    let html = "<h1>Welcome to Cooking</h1>".to_string();
    let attachments = None;

    let email_sent_status = send_email_html(client, organization, to, subject, html, attachments)
        .await
        .unwrap();

    println!("Email sent with ID: {}", email_sent_status);
}
