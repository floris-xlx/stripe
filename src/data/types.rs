//! ## Global data types
//!

pub struct User {
    name: String,
    email: String,
    country: String,
    created_at: i64,
    customer_id: String,
}

pub struct Payment {
    amount: f64,
    paid: bool,
    receipt_url: String,
    receipt: String,
    email_sent: bool,
    created_at: i64,
}

pub struct Subscription {
    product_id: String,
    id: String,
    customer_id: String,
    created_at: i64,
    start_time: i64,
    end_time: i64,
}
