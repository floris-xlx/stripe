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


pub mod router;


/// ## EventHandler
/// This enum represents the different types of events that can be handled by the event handlers
///
/// ## Variants
/// - `PaymentIntent` - Represents the `payment_intent` event
/// - `Charge` - Represents the `charge` event
/// - `Checkout` - Represents the `checkout` event
/// - `Unknown` - Represents an unknown event
#[derive(Debug, Clone)]
pub enum EventHandler {
    PaymentIntentSucceeded,
    PaymentIntentPaymentFailed,
    PaymentIntentCreated,
    CheckoutSessionCompleted,
    ChargeSucceeded,
    ChargeFailed,
    Unknown
}


pub struct CheckoutSessionCompleted {
    pub email: String,
    pub name: String,
    pub amount: i64,
    pub paid_status: bool,
    pub created_at: i64,
    pub country: String,
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
pub struct PaymentIntentPaymentFailed {
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
pub struct PaymentIntentCreated {
    pub country: String,
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
pub struct PaymentIntentSucceeded {
    pub created_at: i64,
    pub paid_status: bool,
    pub amount: i64,
}


/// ## Succeeded
/// This struct represents the `charge.succeeded` event
///
/// ### Fields
/// - `email` - The email of the user
/// - `name` - The name of the user
/// - `amount` - The amount of the payment_intent
/// - `paid_status` - The paid status of the user
/// - `receipt_url` - The receipt receipt_url
/// - `created_at` - The created at timestamp
/// - `country` - The country of the user
///
#[derive(Debug, Clone)]
pub struct ChargeSucceeded {
    pub email: String,
    pub name: String,
    pub amount: i64,
    pub paid_status: bool,
    pub receipt_url: String,
    pub created_at: i64,
    pub country: String,
}



/// ## Charge.failed event scope
/// This struct represents the `charge.succeeded` event
///
/// ### Fields
/// - `paid_status` - The paid status of the user
#[derive(Debug, Clone)]
pub struct ChargeFailed {
    pub paid_status: bool,
}