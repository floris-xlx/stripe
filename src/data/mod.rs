//! ## Data structures and types

pub mod types;

use crate::data::types::{Payment, Subscription, User};

/// ## Custom data Types
///
/// This enum represents the different types of data that can be handled by the application
///
/// ## Variants
/// - `User` - Represents the user data
/// - `Payment` - Represents the payment Data
/// - `Subscription` - Represents the subscription data
///
pub enum Types {
    User(User),
    Payment(Payment),
    Subscription(Subscription),
}
