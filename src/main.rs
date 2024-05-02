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
use stripe_discord::Organization;
use stripe_discord::EmailConfig;


#[post("/stripe_webhooks", format = "json", data = "<webhook_data>")]
pub async fn stripe_webhook(
    webhook_data: Json<Value>,
) -> status::Custom<String> {
    println!("JSON Payload: {:#?}", webhook_data);


    // build the email config
    let email_config: EmailConfig = EmailConfig::new(
        "billing@xylex.cloud".to_string(),
        "Welcome to Xylex Enterprise!".to_string(),
        "xxxx".to_string(),
    );

    // build the organization
    let organization: Organization = Organization::new(
        "Xylex".to_string(),
        email_config
    );
    // build the event handler
    let event_handler: EventHandler = EventHandler::new(
        &webhook_data,
        organization
    ).await;





    println!("\x1b[32mEvent Handler: {:#?}\x1b[0m", event_handler);
    status::Custom(
        Status::Ok,
        "Received webhook".to_string()
    )
}
