//! ## PaymentIntent event handler & wrapper
//!
//! This module contains the event handler for the `payment_intent` event and a wrapper for the event handler.
//!
//! ### Handled event objects
//! - [`payment_intnet.succeeded`]
//! - [`payment_intent.payment_failed`]
//! - [`payment_intent.created`]
//!
//!

use serde_json::Value;

// import the data Types

use crate::events::PaymentIntent;

#[derive(Debug, Clone)]
pub enum PaymentIntentEvents {
    Succeeded,
    PaymentFailed,
    Created,
}

/// ## PaymentIntent.succeeded event scope
/// This struct represents the `payment_intent.succeeded` event
///
/// ### Fields
/// - `created_at` - The created at timestamp
/// - `paid_status` - The paid status of the user
/// - `amount` - The amount of the payment_intent
///
#[derive(Debug, Clone)]
pub struct Succeeded {
    pub created_at: i64,
    pub paid_status: bool,
    pub amount: i64,
}

/// ## PaymentIntent.payment_failed event scope
/// This struct represents the `payment_intent.payment_failed` event
///
/// ### Fields
/// - `email` - The email of the user
/// - `name` - The name of the user
/// - `paid_status` - The paid status of the user
/// - `created_at` - The created at timestamp
/// - `country` - The country of the user
#[derive(Debug, Clone)]
pub struct PaymentFailed {
    pub email: String,
    pub name: String,
    pub paid_status: bool,
    pub created_ad: i64,
    pub country: String,
}

/// ## PaymentIntent.created event scope
/// This struct represents the `payment_intent.created` event
///
/// ### Fields
/// - `country` - The country of the payment intent
#[derive(Debug, Clone)]
pub struct Created {
    pub country: String,
}

impl PaymentIntent {
    /// ## payment_intent.succeeded
    ///
    /// ### Example
    /// ```rust
    /// use stripe_discord::events::PaymentIntent;
    ///
    /// let payment_intent = PaymentIntent WIP WIP WIP
    /// ```
    pub fn succeeded(&self) {
        println!("Payment intent succeeded: {}", self.event_title);
    }

    /// ## payment_intent.payment_failed
    /// This function is called when the payment intent payment fails
    ///
    /// ### Example
    /// ```rust
    /// use stripe_discord::events::PaymentIntent;
    ///
    /// let payment_intent = PaymentIntent WIP WIP WIP
    /// ```
    ///
    pub fn payment_failed(&self) {
        println!("Payment intent payment failed: {}", self.event_title);
    }

    /// ## payment_intent.created
    /// This function is called when the payment intent is created
    ///
    /// ### Example
    /// ```rust
    /// use stripe_discord::events::PaymentIntent;
    ///
    /// let payment_intent = PaymentIntent WIP WIP WIP
    /// ```
    ///
    ///
    pub fn created(&self) {
        println!("Payment intent created: {}", self.event_title);
    }
}
