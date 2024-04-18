use stripe_discord::Config;



fn main() {
    println!("Hello, world!");

    let config = Config::new("stripe_discord.yaml").unwrap();
}
