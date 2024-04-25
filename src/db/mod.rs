//! ## Database handling and logic
//! Here we have the database logic for the API.
//!
//! ### Db providers
//! - [SupabaseDb](struct.SupabaseDb.html)
//! - [SledDb](struct.SledDb.html)
//!
//! ### Modules
//! - [format](format/index.html)
//! - [sled](sled/index.html)
//! - [supabase](supabase/index.html)
//!
use supabase_rs::SupabaseClient;

pub mod format;
pub mod sled;
pub mod supabase;

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
    pub db: String, // FIXME this will become the sled type soon
}
