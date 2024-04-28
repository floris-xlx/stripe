//! # CustomerId database operations
//!
//! This module contains the database operations for the customer_id table.
//!
//! ## Table of contents
//!
//!
//! ## Implementations

#![allow(unused_variables)]

use crate::CustomerId;

// temp dev import
use crate::overwrite::{
    overwrite_stripe_customer_id_column_name, overwrite_stripe_customer_table_name,
    overwrite_stripe_email_column_name,
};

use serde_json::json;
use serde_json::Value;
use std::error::Error;
use supabase_rs::SupabaseClient;

impl CustomerId {
    /// ## Attach or overwrite an `EmailAddress` to a `CustomerId` in Supabase
    /// This will attach or overwrite an `EmailAddress` to a `CustomerId` in Supabase
    ///
    /// ### Parameters
    /// - `customer_id` - The `CustomerId` to attach the `EmailAddress` to in Supabase
    /// - `email` - The `EmailAddress` to attach to the `CustomerId` in Supabase
    ///
    /// ### Returns
    /// The `CustomerId` with the attached `EmailAddress` in Supabase
    ///
    /// ### Examples
    /// ```rust
    /// use stripe_discord::CustomerId;
    /// use supabase_rs::SupabaseClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let customer_id = CustomerId::new();
    ///    let email = "floris@xylex.ai";
    ///    let supabase = SupabaseClient::new("SUPABASE_URL", "SUPABASE_KEY");
    ///    let customer_id = CustomerId::attach_email(customer_id, email.to_string(), supabase).await.unwrap();
    ///
    ///    assert_eq!(customer_id.email, email);
    ///}
    ///```
    ///
    pub async fn attach_email(
        customer_id: CustomerId,
        email: String,
        supabase: SupabaseClient,
    ) -> Result<CustomerId, Box<dyn Error>> {
        // temp check if the table name and column names are being overwritten
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_email: String = overwrite_stripe_email_column_name();
        let column_name_customer_id: String = overwrite_stripe_customer_id_column_name();

        let result_row_id: Vec<Value> = supabase
            .select(&table_name)
            .eq(&column_name_customer_id, customer_id.as_str())
            .execute()
            .await
            .unwrap();

        // if result_row_id returns an empty list this means we can just upsert
        if result_row_id.is_empty() {
            let result_email_attach: String = supabase
                .insert(
                    &table_name,
                    json!({
                        column_name_customer_id: customer_id.id,
                        column_name_email: email
                    }),
                )
                .await?;
            return Ok(customer_id);
        }

        let customer_id_from_result: &Value = result_row_id.first().unwrap();

        let id_key: String = customer_id_from_result["id"].as_i64().unwrap().to_string();

        // update the email in supabas if applicable
        let result_email_attach: () = supabase
            .update(
                &table_name,
                &id_key,
                json!({
                    "id": id_key,
                    column_name_customer_id: customer_id.id,
                    column_name_email: email
                }),
            )
            .await?;

        Ok(customer_id)
    }

    /// ## Get Email by `customerId`
    pub async fn get_email(
        customer_id: CustomerId,
        supabase: SupabaseClient,
    ) -> Result<String, Box<dyn Error>> {
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_email: String = overwrite_stripe_email_column_name();
        let column_name_customer_id: String = overwrite_stripe_customer_id_column_name();

        let result_get_email = supabase
            .select(&table_name)
            .eq(&column_name_customer_id, customer_id.as_str())
            .execute()
            .await
            .unwrap()
            ;

        println!("{:#?}", result_get_email);

        let customer_data_from_result: &Value = result_get_email.first().expect("Failed extracting email from Supabase response for `get_email`");

        let email: String = customer_data_from_result["email"]
            .as_str()
            .unwrap()
            .to_string();

        Ok(email)
    }
}
