use crate::events::EventHandler;
use crate::CustomerId;
use crate::utils::format::format_total_amount;


use serde_json::Value;
use supabase_rs::SupabaseClient;
use std::env::var;
use dotenv::dotenv;
use std::thread::sleep;
use std::time::Duration;


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
            "payment_intent.created" => { 
                // ghost event, pretty irrelevant for now

                EventHandler::PaymentIntentCreated 
            },
            "payment_intent.payment_failed" => { EventHandler::PaymentIntentPaymentFailed },
            "payment_intent.succeeded" => { EventHandler::PaymentIntentSucceeded },
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

                // unwrapped amount_captured
                let amount_captured: i64 = object.get("amount_captured")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);

                CustomerId::update_amount_total(
                    CustomerId {id: customer_id.clone()}, 
                    format_total_amount(amount_captured).await, 
                    supabase.clone()
                ).await.unwrap();

                // unwrapped country
                let country: String = object.get("billing_details")
                    .and_then(|billing_details| billing_details.get("address"))
                    .and_then(|address| address.get("country"))
                    .and_then(|country| country.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                CustomerId::update_country(
                    CustomerId {id: customer_id.clone()}, 
                    country, 
                    supabase.clone()
                ).await.unwrap();

                // unwrapped receipt_url
                let receipt_url: String = object.get("receipt_url")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                CustomerId::update_receipt_url(
                    CustomerId {id: customer_id.clone()}, 
                    receipt_url, 
                    supabase.clone()
                ).await.unwrap();

                // payment status status: succeeded
                let payment_status: String = object.get("status")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                if payment_status == "succeeded" {
                    CustomerId::update_paid(
                        CustomerId {id: customer_id.clone()}, 
                        true, 
                        supabase.clone()
                    ).await.unwrap();

                } else {
                    CustomerId::update_paid(
                        CustomerId {id: customer_id.clone()}, 
                        false, 
                        supabase.clone()
                    ).await.unwrap();
                }


                EventHandler::ChargeSucceeded
            },
            "charge.failed" => { EventHandler::ChargeFailed },
            "checkout.session.completed" => { 
                
                // unwrap payment_link
                let payment_link: String = object.get("payment_link")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                // unwrap email
                let email: String = object.get("customer_details")
                    .and_then(|customer_details| customer_details.get("email"))
                    .and_then(|email| email.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                CustomerId::cache_payment_link(
                    email,
                    payment_link, 
                    supabase.clone()
                ).await.unwrap();
                
                

                
                EventHandler::CheckoutSessionCompleted 
            },
            _ => EventHandler::Unknown,
        }
    }
}
