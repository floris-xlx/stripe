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


/// # init_supabase_client
/// Initializes and returns a new instance of the Supabase client configured with environment variables.
///
/// ## Arguments
/// This function does not take any arguments directly. However, it retrieves the following from environment variables:
/// - `SUPABASE_URL`: The URL of the Supabase project.
/// - `SUPABASE_KEY`: The secret key used for authenticating with the Supabase API.
///
/// ## Returns
/// Returns a `SupabaseClient` object configured with the specified URL and key.
/// If the environment variables are not set, the function will panic with an error message.
///
/// ## Example: Initializing a Supabase client
/// ```
/// let client = init_supabase_client();
/// // Now `client` can be used to interact with Supabase services.
/// ```
pub fn init_supabase_client() -> SupabaseClient {
    let supabase_url: String = var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let supabase_key: String = var("SUPABASE_KEY").expect("SUPABASE_KEY must be set");

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
