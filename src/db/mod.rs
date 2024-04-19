//! ## Databasing 
//! Here we have the database logic for the API.
//! 
//! ### Db variants
//! - `sled`
//! - `supabase`
//! 
use supabase_rs::SupabaseClient;


pub mod sled;
pub mod supabase;
pub mod format;


/// ## Supabase
/// 
/// This is the supabase database.
#[derive(Debug, Clone)]
pub struct SupabaseDb {
    pub supabase: SupabaseClient,
    pub supabase_url: String,
    pub supabase_key: String,
}


/// ## Sled
/// 
/// This is the sled database.
#[derive(Debug, Clone)]
pub struct SledDb {
    pub db: String // FIXME this will become the sled type soon
}

