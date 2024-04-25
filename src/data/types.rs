//! ## Global data types
//!

pub enum Types {
    User,
    Payment,
    Subscription,
}

pub struct User {
    name: String,
    email: String,
    country: String,
    created_at: String,
    customer_id: String,
}

pub struct Payment {
    amount: f64,
    paid: bool,
    receipt_url: String,
    receipt: String,
    email_sent: bool,
    created_at: String,
}

pub struct Subscription {
    product_id: String,
    customer_id: String,
    created_at: String,
    start_time: String,
    end_time: String,
}
