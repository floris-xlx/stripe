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
use std::env::var;
use std::error::Error;
use supabase_rs::SupabaseClient;

impl CustomerId {
    /// ## `new` that initializes a new instance and inserts the `CustomerId` to supabase with
    /// nothing else
    ///
    pub async fn new(customer_id: CustomerId) -> Result<CustomerId, Box<dyn Error>> {
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_customer_id: String = overwrite_stripe_customer_id_column_name();

        // init the supabase client using the standard env vars
        let supabase =
            SupabaseClient::new(var("SUPABASE_URL").unwrap(), var("SUPABASE_KEY").unwrap());

        let result_customer_id: String = supabase
            .insert(
                &table_name,
                json!({
                    column_name_customer_id: customer_id.id
                }),
            )
            .await?;

        Ok(customer_id)
    }

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
        // temp check if the tabVle name and column names are being overwritten
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

    /// # get_email
    /// Retrieves the email address associated with a given `CustomerId` from the Supabase database.
    ///
    /// ## Arguments
    /// - `customer_id`: `CustomerId` - The unique identifier for the customer whose email is being retrieved.
    /// - `supabase`: `SupabaseClient` - The client used to interact with the Supabase database.
    ///
    /// ## Returns
    /// - `Result<String, Box<dyn Error>>`: This function returns a `Result` which is either:
    ///   - `Ok(String)`: The email address of the customer if found.
    ///   - `Err(Box<dyn Error>)`: An error boxed in a trait object if an issue occurs during the database operation.
    ///
    /// ## Examples
    /// ```rust
    /// async fn example_usage() -> Result<(), Box<dyn Error>> {
    ///     let customer_id = CustomerId::new("some_unique_id");
    ///     let supabase_client = SupabaseClient::new("your_supabase_url", "your_supabase_key");
    ///     let email = get_email(customer_id, supabase_client).await?;
    ///     println!("Email: {}", email);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_email(
        customer_id: CustomerId,
        supabase: SupabaseClient,
    ) -> Result<String, Box<dyn Error>> {
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_email: String = overwrite_stripe_email_column_name();
        let column_name_customer_id: String = overwrite_stripe_customer_id_column_name();

        let result_get_email: Vec<Value> = supabase
            .select(&table_name)
            .eq(&column_name_customer_id, customer_id.as_str())
            .execute()
            .await
            .unwrap();

        println!("{:#?}", result_get_email);

        let customer_data_from_result: &Value = result_get_email
            .first()
            .expect("Failed extracting email from Supabase response for `get_email`");

        let email: String = customer_data_from_result["email"]
            .as_str()
            .unwrap()
            .to_string();

        Ok(email)
    }
}
