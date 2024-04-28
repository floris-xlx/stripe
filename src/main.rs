use stripe_discord::email::resend::authenticate;
use stripe_discord::email::resend::send_email_html;

use stripe_discord::Organization;

use supabase_rs::SupabaseClient;

use stripe_discord::CustomerId;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("Hello, world!");

    use dotenv::dotenv;
    dotenv().ok();

    let supabase_client = SupabaseClient::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    );

    let customer_id = CustomerId::new("customer_cooking".to_string());
    let email = "hadi@xylex.ai".to_string();

    // let customer_id = CustomerId::attach_email(customer_id.clone(), email, supabase_client.clone())
    // .await
    //.unwrap();

    let customer_email = CustomerId::get_email(customer_id.clone(), supabase_client.clone())
        .await
        .unwrap();

    println!("Customer email: {:?}", customer_email);

    println!("Supabase client: {:?}", supabase_client);
    println!("Customer ID: {:?}", customer_id);
}
