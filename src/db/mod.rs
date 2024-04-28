//! ## Database handling and logic
//! Here we have the database logic for the API.
//!
//! ### Pre-requisites supabase table names and columns
//! So as we heavily rely on Supabase for databasing we will need to have the following tables and columns:
//! #### Tables  
//! - `stripe_customer_data` - The table to store the customer database
//!
//! #### `stripe_customer_data` columns
//! - `customer_id` TYPE TEXT - The customer ID from Stripe
//! - `email` TYPE TEXT - The email address of the customer
//!
//!
//! ### Db providers
//! - [SupabaseDb](struct.SupabaseDb.html)
//!
//! ### Modules
//! - [format](format/index.html)
//! - [supabase](supabase/index.html)
//! - [operations](operations/index.html)
//!
//!
use std::env::var;
use supabase_rs::SupabaseClient;

pub mod format;
pub mod operations;
pub mod supabase;

/// ## Initialize the Supabase client
/// This function initializes the Supabase client
///
pub fn init_supabase_client() -> SupabaseClient {
    let supabase_url = var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let supabase_key = var("SUPABASE_KEY").expect("SUPABASE_KEY must be set");

    SupabaseClient::new(supabase_url, supabase_key)
}

/// ## CRUD operations
///
/// This struct represents the CRUD operations
///
/// ### Fields
/// - `create` - The create operations
/// - `read` - The read operations
/// - `update` - The update operations
/// - `delete` - The delete operations
///
#[derive(Debug, Clone)]
pub struct CRUD {
    pub create: String,
    pub read: String,
    pub update: String,
    pub delete: String,
}
