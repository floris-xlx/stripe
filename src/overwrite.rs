//! # Overwriting Default Values for Stripe Customer Data

use dotenv::dotenv;
use std::env::var;

/// ## Overwrite Stripe Customer Table Name
///
/// This function will return the table name to use in Supabase for the Stripe Customer data
///
/// ### Returns
/// The table name to use in Supabase for the Stripe Customer data
///
/// ### Examples
/// ```rust
/// use stripe_discord::overwrite::overwrite_stripe_customer_table_name;
///
/// let table_name = overwrite_stripe_customer_table_name();
/// ```
///
pub fn overwrite_stripe_customer_table_name() -> String {
    dotenv().ok();

    let table_name: String = match var("OVERWRITE_STRIPE_CUSTOMER_TABLE_NAME") {
        Ok(table_name) => table_name.clone(),
        Err(_) => "stripe_customer_data".to_string(),
    };

    table_name
}

/// ## Overwrite Stripe Email Column Name
///
/// This function will return the column name for the email in Supabase for the Stripe Customer data
///
/// ### Returns
/// The column name for the email to use in Supabase for the Stripe Customer data
///
/// ### Examples
/// ```rust
/// use stripe_discord::overwrite::overwrite_stripe_email_column_name;
///
/// let column_name_email = overwrite_stripe_email_column_name();
/// ```
///
pub fn overwrite_stripe_email_column_name() -> String {
    dotenv().ok();

    let column_name_email: String = match var("OVERWRITE_STRIPE_EMAIL_COLUMN_NAME") {
        Ok(column_name_email) => column_name_email.clone(),
        Err(_) => "email".to_string(),
    };

    column_name_email
}

/// ## Overwrite Stripe Customer ID Column Name
///
/// This function will return the column name for the customer ID in Supabase for the Stripe Customer data
///
/// ### Returns
/// The column name for the customer ID to use in Supabase for the Stripe Customer data
///
/// ### Examples
/// ```rust
/// use stripe_discord::overwrite::overwrite_stripe_customer_id_column_name;
///
/// let column_name_customer_id = overwrite_stripe_customer_id_column_name();
/// ```
///
pub fn overwrite_stripe_customer_id_column_name() -> String {
    dotenv().ok();

    let column_name_customer_id: String = match var("OVERWRITE_STRIPE_CUSTOMER_ID_COLUMN_NAME") {
        Ok(column_name_customer_id) => column_name_customer_id.clone(),
        Err(_) => "customer_id".to_string(),
    };

    column_name_customer_id
}

/// ## Overwrite Stripe Customer Paid column name
/// Type: Boolean
///
/// This function will return the column name for the customer paid status in Supabase for the Stripe Customer data
///
/// ### Returns
/// The column name for the customer paid status to use in Supabase for the Stripe Customer data
///  
///  ### Examples
///  ```rust
///  use stripe_discord::overwrite::overwrite_stripe_customer_paid_column_name;
///
///  let column_name_customer_paid = overwrite_stripe_customer_paid_column_name();
///  ```
///
pub fn overwrite_stripe_customer_paid_column_name() -> String {
    dotenv().ok();

    let column_name_customer_paid: String = match var("OVERWRITE_STRIPE_CUSTOMER_PAID_COLUMN_NAME")
    {
        Ok(column_name_customer_paid) => column_name_customer_paid.clone(),
        Err(_) => "paid".to_string(),
    };

    column_name_customer_paid
}
