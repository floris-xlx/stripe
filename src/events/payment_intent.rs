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
use crate::data::types::{Payment, Subscription, User};

use crate::events::PaymentIntent;

pub enum PaymentIntentEvents {
    Succeeded,
    PaymentFailed,
    Created,
}

/// ## PaymentIntent.succeeded event scope
pub struct Succeeded {
    pub event_title: String,

    // the event_object captures the raw data object we get from stripe
    pub event_object: Value,
    pub created_at: i64,
}

impl PaymentIntent {
    /// ## payment_intent.succeeded
    pub fn succeeded(&self) {
        println!("Payment intent succeeded: {}", self.event_title);
    }

    /// ## payment_intent.payment_failed
    /// This function is called when the payment intent payment fails
    pub fn payment_failed(&self) {
        println!("Payment intent payment failed: {}", self.event_title);
    }

    /// ## payment_intent.created
    /// This function is called when the payment intent is created
    pub fn created(&self) {
        println!("Payment intent created: {}", self.event_title);
    }
}
