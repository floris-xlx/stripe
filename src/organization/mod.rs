//! ## Creating different Organization profiles under different stripe accounts
//! The `StripeEndpointConfig` takes different configurations, this can be manually initialized via
//! a struct as described in LIB.RS, But you can also create Organizations to easily inject your
//! `stripe` with the correct data and utilize tests when doing so
//!
//! ### Table of contents
//! - What are Organizations
//! - Why would i use them
//! - How to create a new Organization
//! - How do i inject an Organization into my `StripeEndpointConfig`
//!
//!

use crate::EndpointConfigStripe;
use crate::Organization;

pub mod model;
pub mod router;


/// ## Organization implementation `new`
/// This is the implementation of the `Organization` struct
///
impl Organization {
    /// # new
    /// Constructs a new `Organization` instance with specified name and sender email.
    ///
    /// ## Arguments
    /// - `name`: `String` - The name of the Organization.
    /// - `sender_email`: `String` - The email address associated with the Organization.
    ///
    /// ## Returns
    /// - `Organization`: Returns a new instance of `Organization` populated with the provided name and email.
    ///
    /// ## Examples
    /// ```
    /// use crate::Organization;
    ///
    /// let org = Organization::new("Acme Corp".to_string(), "contact@acmecorp.com".to_string());
    /// println!("Organization created: {} with email {}", org.name, org.sender_email);
    /// ```
    pub fn new(
        name: String, 
        sender_email: String
    ) -> Organization {

        // return the new instance of Organization
        Organization { 
            name, 
            sender_email 
        }
    }

}
