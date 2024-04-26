//! ## Checkout event handler & wrapper
//!
//! This module contains the event handler for the `checkout` event and a wrapper for the event handler.
//!
//! ### Handled event objects
//! - [`checkout.session.completed`]
//!

use crate::events::Checkout;

/// ## CheckoutEvent
///
/// This enum represents the different types of events that can be handled by the checkout event handler
///
/// ## Variants
/// - `SessionCompleted` - Represents the `checkout.session.completed` event
#[derive(Debug, Clone)]
pub enum CheckoutEvent {
    SessionCompleted,
}

/// ## SessionCompleted
/// This struct represents the `checkout.session.completed` event
///
/// ### Fields
/// - `email` - The email of the user
/// - `name` - The name of the user
/// - `amount` - The amount of the payment_intent
/// - `paid_status` - The paid status of the user
/// - `created_at` - The created at timestamp
/// - `country` - The country of the user
#   
pub struct SessionCompleted {
    pub email: String,
    pub name: String,
    pub amount: i64,
    pub paid_status: bool,
    pub created_at: i64,
    pub country: String,
}

impl Checkout {
    /// ## checkout.session.completed
    ///
    /// ### Example
    /// ```rust
    /// use stripe_discord::events::Checkout;
    ///
    /// let checkout = Checkout WIP WIP WIP
    /// ```
    pub fn session_completed(&self) {
        println!("Checkout session completed: {}", self.event_title);
    }
}
