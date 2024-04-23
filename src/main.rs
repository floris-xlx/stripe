use stripe_discord::ConfigSetup;

// ## Main call
// EXPLANATION OF FN
//
//
// ### Usage example
// ```rust
//
//
// ```
//
// ### Arguments
//
// ### Returns
//
// ### Errors
// errors will be dealt with by errors.rs
//
//
// ### Notes
//
//
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("Hello, world!");

    // create the config instance
    let config = ConfigSetup::new();

    println!("{:#?}", config)
}
