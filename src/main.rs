use stripe_discord::ConfigSetup;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("Hello, world!");

    // create the config instance
    let config = ConfigSetup::new();

    println!("{:#?}", config)
}
