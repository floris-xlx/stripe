use crate::events::EventHandler;
use crate::CustomerId;



use serde_json::Value;
use supabase_rs::SupabaseClient;
use std::env::var;
use dotenv::dotenv;


impl EventHandler {
    pub async fn new(json_data: &Value) -> Self {
        dotenv().ok();
    
        let event_type: &str = json_data.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");

        // init the supabase client using the standard env vars
        let supabase: SupabaseClient =
            SupabaseClient::new(var("SUPABASE_URL").unwrap(), var("SUPABASE_KEY").unwrap());


        // unwrapped object
        let object = json_data.get("data")
            .and_then(|data| data.get("object"))
            .unwrap_or(&Value::Null);

        match event_type {
            "payment_intent.created" => EventHandler::PaymentIntentCreated,
            "payment_intent.payment_failed" => EventHandler::PaymentIntentPaymentFailed,
            "payment_intent.succeeded" => EventHandler::PaymentIntentSucceeded,
            "charge.succeeded" => {

                // unwrapped customer_id
                let customer_id: String = object.get("id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                // create a blank CustomerId object in db
                CustomerId::new(
                    CustomerId {id: customer_id.clone()}, 
                    true, 
                    supabase.clone()
                ).await.unwrap();

                // unwrapped email
                let email: String = object.get("billing_details")
                    .and_then(|billing_details| billing_details.get("email"))
                    .and_then(|email| email.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                CustomerId::attach_email(
                    CustomerId {id: customer_id.clone()}, 
                    email, 
                    supabase.clone()
                ).await.unwrap();

                // unwrapped name
                let name: String = object.get("billing_details")
                    .and_then(|billing_details| billing_details.get("name"))
                    .and_then(|name| name.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                CustomerId::update_name(
                    CustomerId {id: customer_id.clone()}, 
                    name, 
                    supabase.clone()
                ).await.unwrap();

                // unwrapped amount_total
                let amount_total: i64 = object.get("amount_total")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);







                EventHandler::ChargeSucceeded
            },
            "charge.failed" => EventHandler::ChargeFailed,
            "checkout.session.completed" => EventHandler::CheckoutSessionCompleted,
            _ => EventHandler::Unknown,
        }
    }
}
