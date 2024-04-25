//! ## Charge event handler & wrapper
//!
//! This module contains the event handler for the `charge` event and a wrapper for the event handler.
//!
//! ### Handled event objects
//! - [`charge.succeeded`]
//! - [`charge.failed`]
//!

use crate::events::Charge;

impl Charge {
    /// ## charge.succeeded
    pub fn succeeded(&self) {
        println!("Charge succeeded: {}", self.event_title);
    }

    /// ## charge.failed
    /// This function is called when the charge fails
    pub fn failed(&self) {
        println!("Charge failed: {}", self.event_title);
    }
}
