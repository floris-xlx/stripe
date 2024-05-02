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
    overwrite_stripe_customer_email_sent_column_name, 
    overwrite_stripe_customer_id_column_name,
    overwrite_stripe_customer_paid_column_name, 
    overwrite_stripe_customer_table_name,
    overwrite_stripe_email_column_name, 
    overwrite_stripe_customer_end_time_column_name,
    overwrite_stripe_customer_name_column_name, 
    overwrite_stripe_customer_receipt_url_column_name,
    overwrite_stripe_customer_country_column_name,
    overwrite_stripe_customer_amount_total_column_name,
    overwrite_stripe_customer_payment_link_column_name,
    overwrite_stripe_plink_cache_table_name
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
    /// ### Parameters
    /// - `customer_id` - The `CustomerId` to insert into Supabase
    /// - `create_record` - A boolean that determines if a record should be created in Supabase
    /// - `supabase` - The Supabase client to use for the operation
    /// 
    /// ### Returns
    /// The `CustomerId` that was inserted into Supabase
    /// 
    /// ### Example: Creating an instance of `CustomerId` and inserting it into Supabase
    /// ```rust
    /// let customer_id = CustomerId::new();
    /// let customer_id = CustomerId::new(customer_id, true, supabase).await.unwrap();
    /// 
    /// assert_eq!(customer_id.id, "some_unique_id");
    /// ```
    pub async fn new(
        customer_id: CustomerId,
        create_record: bool,
        supabase: SupabaseClient,
    ) -> Result<CustomerId, Box<dyn Error>> {
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_customer_id: String = overwrite_stripe_customer_id_column_name();

        if create_record {
            let existing_record = supabase
                .select(&table_name)
                .eq(&column_name_customer_id, customer_id.as_str())
                .execute()
                .await
                .unwrap();

            if !existing_record.is_empty() {
                return Ok(customer_id);
            }

            let result_customer_id: String = supabase
                .insert(
                    &table_name,
                    json!({
                        column_name_customer_id: customer_id.id
                    }),
                )
                .await?;
        }

        Ok(customer_id)
    }


    /// # create a new `CustomerId` object in the Supabase database
    /// This function creates a new `CustomerId` object in the Supabase database using only the email.
    ///
    /// ## Arguments
    /// - `email`: `String` - The email to be associated with the new `CustomerId` object
    /// - `supabase`: `SupabaseClient` - The client used to interact with the Supabase database
    /// 
    /// ## Returns
    /// - `Result<CustomerId, Box<dyn Error>>`: This function returns a `Result` which is either:
    ///   - `Ok(CustomerId)`: The newly created `CustomerId` object
    ///   - `Err(Box<dyn Error>)`: An error boxed in a trait object if an issue occurs during the database operation.
    /// 
    /// ## Example: Creating a new `CustomerId` object using only the email
    /// ```rust
    /// let email = "floris@xylex.ai";
    /// let supabase = SupabaseClient::new("SUPABASE_URL", "SUPABASE_KEY");
    /// let customer_id = CustomerId::new(email.to_string(), supabase).await.unwrap();
    /// ```
    pub async fn new_from_email(
        email: String,
        create_record: bool,
        supabase: SupabaseClient,
    ) -> Result<String, Box<dyn Error>> {
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_email: String = overwrite_stripe_email_column_name();

        if create_record {
            let existing_record = supabase
                .select(&table_name)
                .eq(&column_name_email, &email.as_str())
                .execute()
                .await
                .unwrap();

            if !existing_record.is_empty() {
                return Ok(email);
            }

            let result_customer_id: String = supabase
                .insert(
                    &table_name,
                    json!({
                        column_name_email: email
                    }),
                )
                .await?;
        }

        Ok(email)
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
    /// ### Example: Attaching an `EmailAddress` to a `CustomerId` in Supabase
    /// ```rust
    /// let customer_id = CustomerId::new();
    /// let email = "floris@xylex.ai";
    /// let supabase = SupabaseClient::new("SUPABASE_URL", "SUPABASE_KEY");
    /// let customer_id = CustomerId::attach_email(customer_id, email.to_string(), supabase).await.unwrap();
    ///
    /// assert_eq!(customer_id.email, email);
    /// ```
    /// ### Notes
    /// The `SUPABASE_URL` and `SUPABASE_KEY` are environment variables that should be set for more details see the [Supabase_rs](https://docs.rs/supabase_rs/0.2.5/supabase_rs/)
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

        // init the supabase client using the standard env vars
        let supabase: SupabaseClient =
            SupabaseClient::new(var("SUPABASE_URL").unwrap(), var("SUPABASE_KEY").unwrap());


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
    /// ## Example: Retrieving the email address associated with a `CustomerId`
    /// ```rust
    /// let customer_id = CustomerId::new("some_unique_id");
    /// let supabase_client = SupabaseClient::new("your_supabase_url", "your_supabase_key");
    /// let email = get_email(customer_id, supabase_client).await?;
    /// assert_eq!(email, "floris@xylex.ai");
    /// ```
    /// 
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


    /// ## update `paid` column by `CustomerId`
    /// Type: Boolean
    ///
    /// ### Arguments
    /// - `customer_id`: `CustomerId` - The unique identifier for the customer whose paid status is being updated.
    /// - `paid`: `bool` - The new paid status to be set for the customer.
    /// - `supabase`: `SupabaseClient` - The client used to interact with the Supabase database.
    /// 
    /// ### Returns
    /// - `Result<String, Box<dyn Error>>`: This function returns a `Result` which is either:
    /// - `Ok(String)`: The result of the update operation.
    /// - `Err(Box<dyn Error>)`: An error boxed in a trait object if an issue occurs during the database operation.
    /// 
    /// ### Example: Updating the paid status associated with a `CustomerId`
    /// ```rust
    /// let customer_id = CustomerId::new("some_unique_id");
    /// let supabase_client = SupabaseClient::new("your_supabase_url", "your_supabase_key");
    /// let result = update_paid(customer_id, true, supabase_client).await?;
    /// assert_eq!(result, "success");
    /// ```
    /// 
    pub async fn update_paid(
        customer_id: CustomerId,
        paid: bool,
        supabase: SupabaseClient,
    ) -> Result<String, Box<dyn Error>> {
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_paid: String = overwrite_stripe_customer_paid_column_name();
        let column_name_customer_id: String = overwrite_stripe_customer_id_column_name();
        let column_name_email: String = overwrite_stripe_email_column_name();

        // fetch email over the self
        let email: String = CustomerId::get_email(customer_id.clone(), supabase.clone())
            .await
            .unwrap();

        let row_id = SupabaseClient::get_id(
            supabase.clone(),
            email,
            table_name.clone(),
            column_name_email,
        );

        let row_id: String = row_id.await.unwrap();

        let result_update_paid: String = supabase
            .upsert(
                &table_name,
                &row_id,
                json!({
                    column_name_paid: paid
                }),
            )
            .await
            .unwrap();

        Ok(result_update_paid)
    }


    /// # get_paid
    /// Retrieves the paid status associated with a given `CustomerId` from the Supabase database.
    ///
    /// ## Arguments
    /// - `customer_id`: `CustomerId` - The unique identifier for the customer whose paid status is being retrieved.
    /// - `supabase`: `SupabaseClient` - The client used to interact with the Supabase database.
    /// 
    /// ## Returns
    /// - `Result<bool, Box<dyn Error>>`: This function returns a `Result` which is either:
    /// - `Ok(bool)`: The paid status of the customer if found.
    /// - `Err(Box<dyn Error>)`: An error boxed in a trait object if an issue occurs during the database operation.
    /// 
    /// ## Example: Retrieving the paid status associated with a `CustomerId`
    /// ```rust
    /// let customer_id = CustomerId::new("some_unique_id");
    /// let supabase_client = SupabaseClient::new("your_supabase_url", "your_supabase_key");
    /// let paid = get_paid(customer_id, supabase_client).await?;
    /// assert_eq!(paid, true);
    /// ```
    /// 
    pub async fn get_paid(
        customer_id: CustomerId,
        supabase: SupabaseClient,
    ) -> Result<bool, Box<dyn Error>> {
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_paid: String = overwrite_stripe_customer_paid_column_name();
        let column_name_customer_id: String = overwrite_stripe_customer_id_column_name();
        let column_name_email: String = overwrite_stripe_email_column_name();

        let result_get_paid: Vec<Value> = supabase
            .select(&table_name)
            .eq(&column_name_customer_id, customer_id.as_str())
            .execute()
            .await
            .unwrap();

        let customer_data_from_result: &Value = result_get_paid
            .first()
            .expect("Failed extracting paid status from Supabase response for `get_paid`");

        let paid: bool = customer_data_from_result["paid"].as_bool().unwrap();

        Ok(paid)
    }


    /// # update `add_email_sent` column by `CustomerId`
    /// Type: Boolean
    ///
    /// ### Arguments
    /// - `customer_id`: `CustomerId` - The unique identifier for the customer whose email sent status is being updated.
    /// - `email_sent`: `bool` - The new email sent status to be set for the customer.
    /// - `supabase`: `SupabaseClient` - The client used to interact with the Supabase database.
    /// 
    /// ### Returns
    /// - `Result<String, Box<dyn Error>>`: This function returns a `Result` which is either:
    /// - `Ok(String)`: The result of the update operation.
    /// - `Err(Box<dyn Error>)`: An error boxed in a trait object if an issue occurs during the database operation.
    /// 
    /// ### Example: Updating the email sent status associated with a `CustomerId`
    /// ```rust
    /// let customer_id = CustomerId::new("some_unique_id");
    /// let supabase_client = SupabaseClient::new("your_supabase_url", "your_supabase_key");
    /// let result = update_email_sent(customer_id, true, supabase_client).await?;
    /// assert_eq!(result, "success");
    /// ```
    /// 
    pub async fn update_email_sent(
        customer_id: CustomerId,
        email_sent: bool,
        supabase: SupabaseClient,
    ) -> Result<String, Box<dyn Error>> {
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_email_sent: String = overwrite_stripe_customer_email_sent_column_name();
        let column_name_customer_id: String = overwrite_stripe_customer_id_column_name();
        let column_name_email: String = overwrite_stripe_email_column_name();

        // fetch email over the self
        let email: String = CustomerId::get_email(
            customer_id.clone(), 
            supabase.clone()
        )
            .await
            .unwrap();

        let row_id = SupabaseClient::get_id(
            supabase.clone(),
            email,
            table_name.clone(),
            column_name_email,
        );

        let row_id: String = row_id.await.unwrap();

        let result_update_email_sent: String = supabase
            .upsert(
                &table_name,
                &row_id,
                json!({
                    column_name_email_sent: email_sent
                }),
            )
            .await
            .unwrap();

        Ok(result_update_email_sent)
    }


    /// # get_email_sent
    /// Retrieves the `email_sent` status associated with a given `CustomerId` from the Supabase database.
    ///
    /// ## Arguments
    /// - `customer_id`: `CustomerId` - The unique identifier for the customer whose paid status is being retrieved.
    /// - `supabase`: `SupabaseClient` - The client used to interact with the Supabase database.
    ///
    /// ## Returns
    /// - `Result<bool, Box<dyn Error>>`: This function returns a `Result` which is either:
    /// - `Ok(bool)`: The email sent status of the customer if found.
    /// - `Err(Box<dyn Error>)`: An error boxed in a trait object if an issue occurs during the database operation.
    /// 
    /// ## Example: Retrieving the email sent status associated with a `CustomerId`
    /// ```rust
    /// let customer_id = CustomerId::new("some_unique_id");
    /// let supabase_client = SupabaseClient::new("your_supabase_url", "your_supabase_key");
    /// let email_sent = get_email_sent(customer_id, supabase_client).await?;
    /// assert_eq!(email_sent, true);
    /// ```
    ///     
    /// 
    pub async fn get_email_sent(
        customer_id: CustomerId,
        supabase: SupabaseClient,
    ) -> Result<bool, Box<dyn Error>> {
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_email_sent: String = overwrite_stripe_customer_email_sent_column_name();
        let column_name_customer_id: String = overwrite_stripe_customer_id_column_name();
        let column_name_email: String = overwrite_stripe_email_column_name();

        let result_get_email_sent: Vec<Value> = supabase
            .select(&table_name)
            .eq(&column_name_customer_id, customer_id.as_str())
            .execute()
            .await
            .unwrap();

        let customer_data_from_result: &Value = result_get_email_sent
            .first()
            .expect("Failed extracting paid status from Supabase response for `get_email_sent`");

        let paid: bool = customer_data_from_result["email_sent"].as_bool().unwrap();

        Ok(paid)
    }

    
    /// # update `add_email_sent` column by `CustomerId`
    /// Type: Boolean
    ///
    /// ### Arguments
    /// - `customer_id`: `CustomerId` - The unique identifier for the customer whose email sent status is being updated.
    /// - `end_time`: `i64` - The new end time to be set for the customer.
    /// - `supabase`: `SupabaseClient` - The client used to interact with the Supabase database.
    /// 
    /// ### Returns
    /// - `Result<String, Box<dyn Error>>`: This function returns a `Result` which is either:
    /// - `Ok(String)`: The result of the update operation.
    /// - `Err(Box<dyn Error>)`: An error boxed in a trait object if an issue occurs during the database operation.
    /// 
    /// ### Example: Updating the email sent status associated with a `CustomerId`
    /// ```rust
    /// let customer_id = CustomerId::new("some_unique_id");
    /// let supabase_client = SupabaseClient::new("your_supabase_url", "your_supabase_key");
    /// let result = update_end_time(customer_id, 11111111111111, supabase_client).await?;
    /// assert_eq!(result, "success");
    /// ```
    /// 
    pub async fn update_end_time(
        customer_id: CustomerId,
        end_time: i64,
        supabase: SupabaseClient,
    ) -> Result<String, Box<dyn Error>> {
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_end_time: String = overwrite_stripe_customer_end_time_column_name();
        let column_name_customer_id: String = overwrite_stripe_customer_id_column_name();
        let column_name_email: String = overwrite_stripe_email_column_name();

        // fetch email over the self
        let email: String = CustomerId::get_email(customer_id.clone(), supabase.clone())
            .await
            .unwrap();

        let row_id = SupabaseClient::get_id(
            supabase.clone(),
            email,
            table_name.clone(),
            column_name_email,
        );

        let row_id: String = row_id.await.unwrap();

        let result_update_end_time: String = supabase
            .upsert(
                &table_name,
                &row_id,
                json!({
                    column_name_end_time: end_time
                }),
            )
            .await
            .unwrap();

        Ok(result_update_end_time)
    }


    /// # get_end_time
    /// Retrieves the `end_time` status associated with a given `CustomerId` from the Supabase database.
    ///
    /// ## Arguments
    /// - `customer_id`: `CustomerId` - The unique identifier for the customer whose paid status is being retrieved.
    /// - `supabase`: `SupabaseClient` - The client used to interact with the Supabase database.
    ///
    /// ## Returns
    /// - `Result<i64, Box<dyn Error>>`: This function returns a `Result` which is either:
    /// - `Ok(i64)`: The end time of the customer if found.
    /// - `Err(Box<dyn Error>)`: An error boxed in a trait object if an issue occurs during the database operation.
    /// 
    /// ## Example: Retrieving the end time associated with a `CustomerId`
    /// ```rust
    /// let customer_id = CustomerId::new("some_unique_id");
    /// let supabase_client = SupabaseClient::new("your_supabase_url", "your_supabase_key");
    /// let end_time = get_end_time(customer_id, supabase_client).await?;
    /// assert_eq!(end_time, 11111111111111);
    /// ```
    /// 
    pub async fn get_end_time(
        customer_id: CustomerId,
        supabase: SupabaseClient,
    ) -> Result<i64, Box<dyn Error>> {
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_email_sent: String = overwrite_stripe_customer_email_sent_column_name();
        let column_name_customer_id: String = overwrite_stripe_customer_id_column_name();
        let column_name_email: String = overwrite_stripe_email_column_name();

        
        let result_get_end_time: Vec<Value> = supabase
            .select(&table_name)
            .eq(&column_name_customer_id, customer_id.as_str())
            .execute()
            .await
            .unwrap();

        let customer_data_from_result: &Value = result_get_end_time
            .first()
            .expect("Failed extracting paid status from Supabase response for `get_end_time`");

        let end_time: i64 = customer_data_from_result["end_time"].as_i64().unwrap();
        Ok(end_time)
    }



    /// ## update_name
    /// Updates the `name` associated with a given `CustomerId` in the Supabase database.
    ///
    /// ### Arguments
    /// - `customer_id`: `CustomerId` - The unique identifier for the customer whose name is being updated.
    /// - `name`: `String` - The new name to be set for the customer.
    /// - `supabase`: `SupabaseClient` - The client used to interact with the Supabase database.
    /// 
    /// ### Returns
    /// - `Result<String, Box<dyn Error>>`: This function returns a `Result` which is either:
    /// - `Ok(String)`: The result of the update operation.
    /// - `Err(Box<dyn Error>)`: An error boxed in a trait object if an issue occurs during the database operation.
    /// 
    /// ### Example: Updating the name associated with a `CustomerId`
    /// ```rust
    /// let customer_id = CustomerId::new("some_unique_id");
    /// let supabase_client = SupabaseClient::new("your_supabase_url", "your_supabase_key");
    /// let result = update_name(customer_id, "New Name", supabase_client).await?;
    /// assert_eq!(result, "success");
    /// ```
    pub async fn update_name(
        customer_id: CustomerId,
        name: String,
        supabase: SupabaseClient,
    ) -> Result<String, Box<dyn Error>> {
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_name: String = overwrite_stripe_customer_name_column_name();
        let column_name_customer_id: String = overwrite_stripe_customer_id_column_name();
        let column_name_email: String = overwrite_stripe_email_column_name();

        // fetch email over the self
        let email: String = CustomerId::get_email(customer_id.clone(), supabase.clone())
            .await
            .unwrap();

        let row_id = SupabaseClient::get_id(
            supabase.clone(),
            email,
            table_name.clone(),
            column_name_email,
        );

        let row_id: String = row_id.await.unwrap();

        let result_update_name: String = supabase
            .upsert(
                &table_name,
                &row_id,
                json!({
                    column_name_name: name
                }),
            )
            .await
            .unwrap();

        Ok(result_update_name)
    }

    /// # get_name
    /// Retrieves the `name` associated with a given `CustomerId` from the Supabase database.
    ///
    /// ## Arguments
    /// - `customer_id`: `CustomerId` - The unique identifier for the customer whose name is being retrieved.
    /// - `supabase`: `SupabaseClient` - The client used to interact with the Supabase database.
    /// 
    /// ## Returns
    /// - `Result<String, Box<dyn Error>>`: This function returns a `Result` which is either:
    /// - `Ok(String)`: The name of the customer if found.
    /// - `Err(Box<dyn Error>)`: An error boxed in a trait object if an issue occurs during the database operation.
    /// 
    /// ## Example: Retrieving the name associated with a `CustomerId`
    /// ```rust
    /// let customer_id = CustomerId::new("some_unique_id");
    /// let supabase_client = SupabaseClient::new("your_supabase_url", "your_supabase_key");
    /// let name = get_name(customer_id, supabase_client).await?;
    /// assert_eq!(name, "John Doe");
    /// ```
    pub async fn get_name(
        customer_id: CustomerId,
        supabase: SupabaseClient,
    ) -> Result<String, Box<dyn Error>> {
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_name: String = overwrite_stripe_customer_name_column_name();
        let column_name_customer_id: String = overwrite_stripe_customer_id_column_name();

        let result_get_name: Vec<Value> = supabase
            .select(&table_name)
            .eq(&column_name_customer_id, customer_id.as_str())
            .execute()
            .await
            .unwrap();

        let customer_data_from_result: &Value = result_get_name
            .first()
            .expect("Failed extracting name from Supabase response for `get_name`");

        let name: String = customer_data_from_result["name"]
            .as_str()
            .unwrap()
            .to_string();

        Ok(name)
    }


    /// # update_receipt_url
    /// Updates the `receipt_url` associated with a given `CustomerId` in the Supabase database.
    /// 
    /// ## Arguments
    /// - `customer_id`: `CustomerId` - The unique identifier for the customer whose receipt_url is being updated.
    /// - `receipt_url`: `String` - The new receipt URL to be associated with the customer.
    /// - `supabase`: `SupabaseClient` - The client used to interact with the Supabase database.
    /// 
    /// ## Returns
    /// - `Result<String, Box<dyn Error>>`: This function returns a `Result` which is either:
    /// - `Ok(String)`: The result of the update operation.
    /// - `Err(Box<dyn Error>)`: An error boxed in a trait object if an issue occurs during the database operation.
    /// 
    /// ## Example: Updating the receipt URL associated with a `CustomerId`
    /// ```rust
    /// let customer_id = CustomerId::new("some_unique_id");
    /// let receipt_url = "https://example.com/receipt";
    /// let supabase_client = SupabaseClient::new("your_supabase_url", "your_supabase_key");
    /// let result = update_receipt_url(customer_id, receipt_url.to_string(), supabase_client).await?;
    /// assert_eq!(result, "success");
    /// ```
    
    /// # get_receipt_url
    /// Retrieves the `receipt_url` associated with a given `CustomerId` from the Supabase database.
    ///
    /// ## Arguments
    /// - `customer_id`: `CustomerId` - The unique identifier for the customer whose receipt_url is being retrieved.
    /// - `supabase`: `SupabaseClient` - The client used to interact with the Supabase database.
    /// 
    /// ## Returns
    /// - `Result<String, Box<dyn Error>>`: This function returns a `Result` which is either:
    /// - `Ok(String)`: The receipt URL of the customer if found.
    /// - `Err(Box<dyn Error>)`: An error boxed in a trait object if an issue occurs during the database operation.
    /// 
    /// ## Example: Retrieving the receipt URL associated with a `CustomerId`
    /// ```rust
    /// let customer_id = CustomerId::new("some_unique_id");
    /// let supabase_client = SupabaseClient::new("your_supabase_url", "your_supabase_key");
    /// let receipt_url = get_receipt_url(customer_id, supabase_client).await?;
    /// assert_eq!(receipt_url, "https://example.com/receipt");
    /// ```
    pub async fn update_receipt_url(
        customer_id: CustomerId,
        receipt_url: String,
        supabase: SupabaseClient,
    ) -> Result<String, Box<dyn Error>> {
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_receipt_url: String = overwrite_stripe_customer_receipt_url_column_name();
        let column_name_customer_id: String = overwrite_stripe_customer_id_column_name();
        let column_name_email: String = overwrite_stripe_email_column_name();

        // fetch email over the self
        let email: String = CustomerId::get_email(customer_id.clone(), supabase.clone())
            .await
            .unwrap();

        let row_id = SupabaseClient::get_id(
            supabase.clone(),
            email,
            table_name.clone(),
            column_name_email,
        );

        let row_id: String = row_id.await.unwrap();

        let result_update_receipt_url: String = supabase
            .upsert(
                &table_name,
                &row_id,
                json!({
                    column_name_receipt_url: receipt_url
                }),
            )
            .await
            .unwrap();

        Ok(result_update_receipt_url)
    }


    /// # get_receipt_url
    /// Retrieves the `receipt_url` associated with a given `CustomerId` from the Supabase database.
    /// 
    /// ## Arguments
    /// - `customer_id`: `CustomerId` - The unique identifier for the customer whose receipt_url is being retrieved.
    /// - `supabase`: `SupabaseClient` - The client used to interact with the Supabase database.
    /// 
    /// ## Returns
    /// - `Result<String, Box<dyn Error>>`: This function returns a `Result` which is either:
    /// - `Ok(String)`: The receipt URL of the customer if found.
    /// 
    /// - `Err(Box<dyn Error>)`: An error boxed in a trait object if an issue occurs during the database operation.
    /// 
    /// ## Example: Retrieving the receipt URL associated with a `CustomerId`
    /// ```rust
    /// let customer_id = CustomerId::new("some_unique_id");
    /// let supabase_client = SupabaseClient::new("your_supabase_url", "your_supabase_key");
    /// let receipt_url = get_receipt_url(customer_id, supabase_client).await?;
    /// assert_eq!(receipt_url, "https://example.com/receipt");
    /// ```
    /// 
    pub async fn get_receipt_url(
        customer_id: CustomerId,
        supabase: SupabaseClient,
    ) -> Result<String, Box<dyn Error>> {
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_receipt_url: String = overwrite_stripe_customer_receipt_url_column_name();
        let column_name_customer_id: String = overwrite_stripe_customer_id_column_name();

        let result_get_receipt_url: Vec<Value> = supabase
            .select(&table_name)
            .eq(&column_name_customer_id, customer_id.as_str())
            .execute()
            .await
            .unwrap();

        let customer_data_from_result: &Value = result_get_receipt_url
            .first()
            .expect("Failed extracting receipt URL from Supabase response for `get_receipt_url`");

        let receipt_url: String = customer_data_from_result["receipt_url"]
            .as_str()
            .unwrap()
            .to_string();

        Ok(receipt_url)
    }


    /// # update_country
    /// Updates the `country` associated with a given `CustomerId` in the Supabase database.
    /// 
    /// ## Arguments
    /// - `customer_id`: `CustomerId` - The unique identifier for the customer whose country is being updated.
    /// - `new_country`: `String` - The new country value to be updated.
    /// - `supabase`: `SupabaseClient` - The client used to interact with the Supabase database.
    /// 
    /// ## Returns
    /// - `Result<(), Box<dyn Error>>`: This function returns a `Result` which is either:
    /// - `Ok(())`: If the country is successfully updated.
    /// 
    /// - `Err(Box<dyn Error>)`: An error boxed in a trait object if an issue occurs during the database operation.
    pub async fn update_country(
        customer_id: CustomerId,
        new_country: String,
        supabase: SupabaseClient,
    ) -> Result<(), Box<dyn Error>> {
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_customer_id: String = overwrite_stripe_customer_id_column_name();
        let column_name_country: String = overwrite_stripe_customer_country_column_name();
        let column_name_email: String = overwrite_stripe_email_column_name();

        // fetch email over the self
        let email: String = CustomerId::get_email(customer_id.clone(), supabase.clone())
            .await
            .unwrap();

        let row_id = SupabaseClient::get_id(
            supabase.clone(),
            email,
            table_name.clone(),
            column_name_email,
        );

        let row_id: String = row_id.await.unwrap();

        let result_update_country: String = supabase
            .upsert(
                &table_name,
                &row_id,
                json!({
                    column_name_country: new_country
                }),
            )
            .await
            .unwrap();

        Ok(())
    }

    /// # get_country
    /// Retrieves the `country` associated with a given `CustomerId` from the Supabase database.
    /// 
    /// ## Arguments
    /// - `customer_id`: `CustomerId` - The unique identifier for the customer whose country is being retrieved.
    /// - `supabase`: `SupabaseClient` - The client used to interact with the Supabase database.
    /// 
    /// ## Returns
    /// - `Result<String, Box<dyn Error>>`: This function returns a `Result` which is either:
    /// - `Ok(String)`: The country of the customer if found.
    /// 
    /// - `Err(Box<dyn Error>)`: An error boxed in a trait object if an issue occurs during the database operation.
    pub async fn get_country(
        customer_id: CustomerId,
        supabase: SupabaseClient,
    ) -> Result<String, Box<dyn Error>> {
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_customer_id: String = overwrite_stripe_customer_id_column_name();
        let column_name_country: String = overwrite_stripe_customer_country_column_name();

        let result_get_country: Vec<Value> = supabase
            .select(&table_name)
            .eq(&column_name_customer_id, customer_id.as_str())
            .execute()
            .await
            .unwrap();

        let customer_data_from_result: &Value = result_get_country
            .first()
            .expect("Failed extracting country from Supabase response for `get_country`");

        let country: String = customer_data_from_result[column_name_country]
            .as_str()
            .unwrap()
            .to_string();

        Ok(country)
    }

    /// ## `update_amount_total`
    /// Updates the `amount_total` associated with a given `CustomerId` in the Supabase database.
    /// 
    /// ### Arguments
    /// - `customer_id`: `CustomerId` - The unique identifier for the customer whose `amount_total` is being updated.
    /// - `new_amount_total`: `f64` - The new `amount_total` to be updated for the customer.
    /// - `supabase`: `SupabaseClient` - The client used to interact with the Supabase database.
    /// 
    /// ### Returns
    /// - `Result<(), Box<dyn Error>>`: This function returns a `Result` which is either:
    ///   - `Ok(())`: If the `amount_total` is successfully updated.
    ///   - `Err(Box<dyn Error>)`: An error boxed in a trait object if an issue occurs during the database operation.
    pub async fn update_amount_total(
        customer_id: CustomerId,
        new_amount_total: f64,
        supabase: SupabaseClient,
    ) -> Result<(), Box<dyn Error>> {
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_customer_id: String = overwrite_stripe_customer_id_column_name();
        let column_name_amount_total: String = overwrite_stripe_customer_amount_total_column_name();
        let column_name_email: String = overwrite_stripe_email_column_name();

        // fetch email over the self
        let email: String = CustomerId::get_email(customer_id.clone(), supabase.clone())
            .await
            .unwrap();

        let row_id = SupabaseClient::get_id(
            supabase.clone(),
            email,
            table_name.clone(),
            column_name_email,
        );


        let row_id: String = row_id.await.unwrap();

        let result_update_amount_total: String = supabase
            .upsert(
                &table_name,
                &row_id,
                json!({
                    column_name_amount_total: new_amount_total
                }),
            )
            .await
            .unwrap();

        Ok(())

    }

    /// ## `get_amount_total`
    /// Retrieves the `amount_total` associated with a given `CustomerId` from the Supabase database.
    /// 
    /// ### Arguments
    /// - `customer_id`: `CustomerId` - The unique identifier for the customer whose `amount_total` is being retrieved.
    /// - `supabase`: `SupabaseClient` - The client used to interact with the Supabase database.
    /// 
    /// ### Returns
    /// - `Result<f64, Box<dyn Error>>`: This function returns a `Result` which is either:
    ///   - `Ok(f64)`: The `amount_total` of the customer if found.
    ///   - `Err(Box<dyn Error>)`: An error boxed in a trait object if an issue occurs during the database operation.
    pub async fn get_amount_total(
        customer_id: CustomerId,
        supabase: SupabaseClient,
    ) -> Result<f64, Box<dyn Error>> {
        let table_name: String = overwrite_stripe_customer_table_name();
        let column_name_customer_id: String = overwrite_stripe_customer_id_column_name();
        let column_name_amount_total: String = overwrite_stripe_customer_amount_total_column_name();

        let result_get_amount_total: Vec<Value> = supabase
            .select(&table_name)
            .eq(&column_name_customer_id, customer_id.as_str())
            .execute()
            .await
            .unwrap();

        let customer_data_from_result: &Value = result_get_amount_total
            .first()
            .expect("Failed extracting amount_total from Supabase response for `get_amount_total`");

        let amount_total: f64 = customer_data_from_result[column_name_amount_total]
            .as_f64()
            .unwrap();

        Ok(amount_total)
    }


    /// # update `payment_link` column by `CustomerId`
    /// Type: String
    ///
    /// ### Arguments
    /// - `customer_id`: `CustomerId` - The unique identifier for the customer whose payment link is being updated.
    /// - `payment_link`: `String` - The new payment link to be set for the customer.
    /// - `supabase`: `SupabaseClient` - The client used to interact with the Supabase database.
    /// 
    /// ### Returns
    /// - `Result<String, Box<dyn Error>>`: This function returns a `Result` which is either:
    /// - `Ok(String)`: The result of the update operation.
    /// - `Err(Box<dyn Error>)`: An error boxed in a trait object if an issue occurs during the database operation.
    /// 
    /// ### Example: Updating the payment link associated with a `CustomerId`
    /// ```rust
    /// let customer_id = CustomerId::new("some_unique_id");
    /// let supabase_client = SupabaseClient::new("your_supabase_url", "your_supabase_key");
    /// let result = update_payment_link(customer_id, "new_payment_link", supabase_client).await?;
    /// assert_eq!(result, "success");
    /// ```
    pub async fn cache_payment_link(
        email: String,
        payment_link: String,
        supabase: SupabaseClient,
    ) -> Result<String, Box<dyn Error>> {
        let table_name: String = overwrite_stripe_plink_cache_table_name();
        let column_name_payment_link: String = overwrite_stripe_customer_payment_link_column_name();
        let column_name_email: String = overwrite_stripe_email_column_name();

        let result_update_payment_link: String = supabase
            .insert(
                &table_name,
                json!({
                    column_name_email: email,
                    column_name_payment_link: payment_link
                }),
            )
            .await
            .unwrap();

        Ok(result_update_payment_link)
    }


    /// # get_payment_link
    /// Retrieves the `payment_link` associated with a given `CustomerId` from the Supabase database.
    ///
    /// ## Arguments
    /// - `email`: `String` - The email of the customer whose payment link is being retrieved.
    /// - `supabase`: `SupabaseClient` - The client used to interact with the Supabase database.
    /// 
    /// ## Returns
    /// - `Result<String, Box<dyn Error>>`: This function returns a `Result` which is either:
    /// - `Ok(String)`: The payment link of the customer if found.
    /// - `Err(Box<dyn Error>)`: An error boxed in a trait object if an issue occurs during the database operation.
    /// 
    /// ## Example: Retrieving the payment link associated with a `CustomerId`
    /// ```rust
    /// let customer_id = CustomerId::new("some_unique_id");
    /// let supabase_client = SupabaseClient::new("your_supabase_url", "your_supabase_key");
    /// let payment_link = get_payment_link(customer_id, supabase_client).await?;
    /// assert_eq!(payment_link, "some_payment_link");
    /// ```
    pub async fn decache_payment_link(
        email: String,
        supabase: SupabaseClient,
    ) -> Result<String, Box<dyn Error>> {
        let table_name: String = overwrite_stripe_plink_cache_table_name();
        let column_name_payment_link: String = overwrite_stripe_customer_payment_link_column_name();
        let column_name_email: String = overwrite_stripe_email_column_name();

        let result_get_payment_link: Vec<Value> = supabase
            .select(&table_name)
            .eq(&column_name_email, email.as_str())
            .execute()
            .await
            .unwrap();

        let customer_data_from_result: &Value = result_get_payment_link
            .first()
            .expect("Failed extracting payment link from Supabase response for `get_payment_link`");

        let payment_link: String = customer_data_from_result[column_name_payment_link]
            .as_str()
            .unwrap()
            .to_string();

        Ok(payment_link)
    }
}
