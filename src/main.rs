#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(unused_parens)]
#![allow(unused_braces)]
#![allow(unused_macros)]
#![allow(unused_import_braces)]



use stripe_discord::email::resend::authenticate;
use stripe_discord::email::resend::send_email_html;
use stripe_discord::Organization;
use stripe_discord::CustomerId;

use rocket::http::Status;
use rocket::Rocket;
use rocket::Build;
use rocket::Config;
use supabase_rs::SupabaseClient;
use rocket::response::{
    status,
    Redirect
};
use rocket::launch;
use rocket::post;
use rocket::routes;
use rocket::{
    Request,
    response::status::Custom,
    response
};
use serde_json::Value;
use rocket::http::Header;
use rocket::response::{Responder, Response};
use rocket::http::HeaderMap;
use rocket::serde::json::Json;
use std::env::var;
use std::env;


#[tokio::main(flavor = "multi_thread")]
async fn cooking() {
    println!("Hello, world!");


    use dotenv::dotenv;
    dotenv().ok();

    let supabase_client: SupabaseClient = SupabaseClient::new(
        var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    );

    // Assuming `stripe_discord::CustomerId` takes a string and CustomerId::new returns a Future which resolves to a String

    let email: String = "floris@xylex.ai".to_string();

    println!("Email: {:?}", email);






}


#[launch]
pub async fn rocket() -> Rocket<Build> {
    // Determine the port to listen on from the `PORT` environment variable, defaulting to 8000.
    let port: u16 = env::var("PORT")
    .unwrap_or_else(|_| String::from("4242"))
    .parse()
    .expect("Failed to parse PORT");

    // Build the Rocket instance, registering error catchers and configuring the server.
    let rocket: Rocket<Build> = rocket::build()
        .configure(Config {
            address: "0.0.0.0".parse().unwrap(), // Listen on all interfaces.
            port,
            ..Config::default()
        })
        .mount("/", routes![ // Mount API routes.
            stripe_webhook
        ]);

    // Return the Rocket instance.
    rocket
}


use stripe_discord::events::EventHandler;


#[post("/stripe_webhooks", format = "json", data = "<webhook_data>")]
pub async fn stripe_webhook(
    webhook_data: Json<Value>,
) -> status::Custom<String> {
    println!("Stripe Webhook received!");
    println!("JSON Payload: {:#?}", webhook_data);


    let event_handler: EventHandler = EventHandler::new(&webhook_data).await;


    println!("\x1b[32mEvent Handler: {:#?}\x1b[0m", event_handler);


    status::Custom(
        Status::Ok,
        "Received webhook".to_string()
    )
}
