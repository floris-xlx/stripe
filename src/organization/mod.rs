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

pub mod model;
pub mod router;
