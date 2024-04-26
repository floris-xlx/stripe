//! ## Charge event handler & wrapper
//!
//! This module contains the event handler for the `charge` event and a wrapper for the event handler.
//!
//! ### Handled event objects
//! - [`charge.succeeded`]
//! - [`charge.failed`]
//!

use crate::events::Charge;

#[derive(Debug, Clone)]
pub enum ChargeEvent {
    Succeeded,
    Failed,
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
pub struct Succeeded {
    pub email: String,
    pub name: String,
    pub amount: i64,
    pub paid_status: bool,
    pub receipt_url: String,
    pub created_at: i64,
    pub country: String,
}

impl Charge {
    /// ## charge.succeeded
    ///
    /// ### Example
    /// ```rust
    /// use stripe_discord::events::Charge;
    ///
    /// let charge = Charge WIP WIP WIP
    /// ```
    pub fn succeeded(&self) {
        println!("Charge succeeded: {}", self.event_title);
    }

    /// ## charge.failed
    /// This function is called when the charge fails
    ///
    /// ### Example
    /// ```rust
    /// use stripe_discord::events::Charge;
    ///
    /// let charge = Charge WIP WIP WIP
    /// ```
    pub fn failed(&self) {
        println!("Charge failed: {}", self.event_title);
    }
}
