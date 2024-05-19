//! ## Email template routing
//!
//! ### How do i add a new email template?


use crate::EmailConfig;

use reqwest::Client;

impl EmailConfig {
    /// # download_email_template
    pub async fn download_email_template(
        &self,
    ) -> Result<String, reqwest::Error> {
        let client: Client = Client::new();
        let email_template_url: String = self.template_url.clone();

        let response: String = client.get(email_template_url)
            .send()
            .await?
            .text()
            .await?;

        println!("Email template downloaded: {}", response);

        Ok(response)
    }

}