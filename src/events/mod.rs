//! ## This module contains all the event handlers for the stripe events.  
//! Each event handler is a module in itself and is responsible for handling the event it is named after.
//!
//! ## Event handlers
//! - [paymentIntent](payment_intent/index.html)
//! - [charge](charge/index.html)
//! - [checkout](checkout/index.html)
//!
//!

use serde_json::Value;

pub mod charge;
pub mod checkout;
pub mod payment_intent;

/// ## EventHandler
/// This enum represents the different types of events that can be handled by the event handlers
///
/// ## Variants
/// - `PaymentIntent` - Represents the `payment_intent` event
/// - `Charge` - Represents the `charge` event
/// - `Checkout` - Represents the `checkout` event
#[derive(Debug, Clone)]
pub enum EventHandler {
    PaymentIntent,
    Charge,
    Checkout,
}

/// ## PaymentIntent
/// This struct represents the `payment_intent` event
#[derive(Debug, Clone)]
pub struct PaymentIntent {
    pub event_title: String,
    pub event_object: Value,
    // pub payment_intent_error: String // this is a placeholder
}

/// ## Charge
/// This struct represents the `charge` event
#[derive(Debug, Clone)]
pub struct Charge {
    pub event_title: String,
    pub event_object: Value,
    // pub charge_error: String // this is a placeholder
}

/// ## Checkout
/// This struct represents the `checkout` event
#[derive(Debug, Clone)]
pub struct Checkout {
    pub event_title: String,
    pub event_object: Value,
    // pub checkout_error: String // this is a placeholder
}
