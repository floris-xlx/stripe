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



/// ### Overwrite `stripe_plink_cache` table name for the Stripe Customer data
///
/// This function will return the table name for the `stripe_plink_cache` in Supabase for the Stripe Customer data
///
/// ### Returns
/// The table name for the `stripe_plink_cache` to use in Supabase for the Stripe Customer data
pub fn overwrite_stripe_plink_cache_table_name() -> String {
    dotenv().ok();

    let table_name: String = match var("OVERWRITE_STRIPE_PLINK_CACHE_TABLE_NAME") {
        Ok(table_name) => table_name.clone(),
        Err(_) => "stripe_plink_cache".to_string(),
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


/// ## Overwrite `email_sent` column name for the Stripe Customer data
///
/// This function will return the column name for the email sent status in Supabase for the Stripe Customer data
///
/// ### Returns  
/// The column name for the email sent status to use in Supabase for the Stripe Customer data  
pub fn overwrite_stripe_customer_email_sent_column_name() -> String {
    dotenv().ok();

    let column_name_customer_email_sent: String =
        match var("OVERWRITE_STRIPE_CUSTOMER_EMAIL_SENT_COLUMN_NAME") {
            Ok(column_name_customer_email_sent) => column_name_customer_email_sent.clone(),
            Err(_) => "email_sent".to_string(),
        };

    column_name_customer_email_sent
}


/// ### Overwrite `end_time` column name for the Stripe Customer data`
///
/// This function will return the column name for the end time in Supabase for the Stripe Customer data
///
/// ### Returns
/// The column name for the end time to use in Supabase for the Stripe Customer data
///
/// ### Examples
/// ```rust
/// use stripe_discord::overwrite::overwrite_stripe_customer_end_time_column_name;
///
/// let column_name_customer_end_time = overwrite_stripe_customer_end_time_column_name();
/// ```
///
pub fn overwrite_stripe_customer_end_time_column_name() -> String {
    dotenv().ok();

    let column_name_customer_end_time: String =
        match var("OVERWRITE_STRIPE_CUSTOMER_END_TIME_COLUMN_NAME") {
            Ok(column_name_customer_end_time) => column_name_customer_end_time.clone(),
            Err(_) => "end_time".to_string(),
        };

    column_name_customer_end_time
}


/// ### Overwrite `name` column name for the Stripe Customer data
///
/// This function will return the column name for the name in Supabase for the Stripe Customer data
///
/// ### Returns
/// The column name for the name to use in Supabase for the Stripe Customer data
pub fn overwrite_stripe_customer_name_column_name() -> String {
    dotenv().ok();

    let column_name_customer_name: String =
        match var("OVERWRITE_STRIPE_CUSTOMER_NAME_COLUMN_NAME") {
            Ok(column_name_customer_name) => column_name_customer_name.clone(),
            Err(_) => "name".to_string(),
        };

    column_name_customer_name
}

/// ### Overwrite `start_time` column name for the Stripe Customer data
///
/// This function will return the column name for the start time in Supabase for the Stripe Customer data
///
/// ### Returns
/// The column name for the start time to use in Supabase for the Stripe Customer data
pub fn overwrite_stripe_customer_start_time_column_name() -> String {
    dotenv().ok();

    let column_name_customer_start_time: String =
        match var("OVERWRITE_STRIPE_CUSTOMER_START_TIME_COLUMN_NAME") {
            Ok(column_name_customer_start_time) => column_name_customer_start_time.clone(),
            Err(_) => "start_time".to_string(),
        };

    column_name_customer_start_time
}



/// ### Overwrite `receipt_url` column name for the Stripe Customer data
///
/// This function will return the column name for the receipt URL in Supabase for the Stripe Customer data
///
/// ### Returns
/// The column name for the receipt URL to use in Supabase for the Stripe Customer data
pub fn overwrite_stripe_customer_receipt_url_column_name() -> String {
    dotenv().ok();

    let column_name_customer_receipt_url: String =
        match var("OVERWRITE_STRIPE_CUSTOMER_RECEIPT_URL_COLUMN_NAME") {
            Ok(column_name_customer_receipt_url) => column_name_customer_receipt_url.clone(),
            Err(_) => "receipt_url".to_string(),
        };

    column_name_customer_receipt_url
}


/// ### Overwrite `country` column name for the Stripe Customer data
///
/// This function will return the column name for the country in Supabase for the Stripe Customer data
///
/// ### Returns
/// The column name for the country to use in Supabase for the Stripe Customer data
pub fn overwrite_stripe_customer_country_column_name() -> String {
    dotenv().ok();

    let column_name_customer_country: String =
        match var("OVERWRITE_STRIPE_CUSTOMER_COUNTRY_COLUMN_NAME") {
            Ok(column_name_customer_country) => column_name_customer_country.clone(),
            Err(_) => "country".to_string(),
        };

    column_name_customer_country
}


/// ### Overwrite `amount_total` column name for the Stripe Customer data
///
/// This function will return the column name for the amount total in Supabase for the Stripe Customer data
///
/// ### Returns
/// The column name for the amount total to use in Supabase for the Stripe Customer data
pub fn overwrite_stripe_customer_amount_total_column_name() -> String {
    dotenv().ok();

    let column_name_customer_amount_total: String =
        match var("OVERWRITE_STRIPE_CUSTOMER_AMOUNT_TOTAL_COLUMN_NAME") {
            Ok(column_name_customer_amount_total) => column_name_customer_amount_total.clone(),
            Err(_) => "amount_total".to_string(),
        };

    column_name_customer_amount_total
}


/// ### Overwrite `payment_link` column name for the Stripe Customer data
///
/// This function will return the column name for the payment link in Supabase for the Stripe Customer data
///
/// ### Returns
/// The column name for the payment link to use in Supabase for the Stripe Customer data
pub fn overwrite_stripe_customer_payment_link_column_name() -> String {
    dotenv().ok();

    let column_name_customer_payment_link: String =
        match var("OVERWRITE_STRIPE_CUSTOMER_PAYMENT_LINK_COLUMN_NAME") {
            Ok(column_name_customer_payment_link) => column_name_customer_payment_link.clone(),
            Err(_) => "payment_link".to_string(),
        };

    column_name_customer_payment_link
}
