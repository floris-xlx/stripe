//! ## Emailing 
//! 
//! ### Providers
//! 
pub mod client;
pub mod resend;
pub mod smtp;
pub mod templates;
pub mod builder;




/// ## Email
/// 
/// 
/// 
#[derive(Debug, Clone)]
pub struct Email {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub body: String,
}

