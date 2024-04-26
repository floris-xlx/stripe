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
    /// ## Create a new Organization
    /// This will create a new Organization with the given `name` and `email`
    ///
    /// ### Parameters
    /// - `name` - The name of the Organization
    /// - `email` - The email of the Organization
    ///
    /// ### Returns
    /// The newly created Organization
    ///
    /// ### Examples (wip)
    pub fn new(name: String, sender_email: String) -> Organization {
        Organization { name, sender_email }
    }
}
