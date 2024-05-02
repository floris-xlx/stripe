//! ## Email template routing
//!
//! ### How do i add a new email template?


use crate::EmailConfig;

use reqwest::Client;

impl EmailConfig {
    pub async fn download_email_template(
        &self,
    ) -> Result<String, reqwest::Error> {
        let client = Client::new();
        let email_template_url = self.template_url.clone();
        let res: String = client.get(email_template_url)
            .send()
            .await?
            .text()
            .await?;

        Ok(res)
    }

}