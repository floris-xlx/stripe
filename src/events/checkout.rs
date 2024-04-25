//! ## Checkout event handler & wrapper
//!
//! This module contains the event handler for the `checkout` event and a wrapper for the event handler.
//!
//! ### Handled event objects
//! - [`checkout.session.completed`]
//!

use crate::events::Checkout;

impl Checkout {
    /// ## checkout.session.completed
    pub fn session_completed(&self) {
        println!("Checkout session completed: {}", self.event_title);
    }
}
