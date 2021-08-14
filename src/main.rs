use dotenv::dotenv;
use egg_mode;
use std::env;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let c_key = env::var("CONSUMER_KEY").expect("Please set CONSUMER_KEY");
    let c_secret = env::var("CONSUMER_SECRET").expect("Please set CONSUMER_SECRET");
    let a_key = env::var("ACCESS_KEY").expect("Please set ACCESS_KEY");
    let a_secret = env::var("ACCESS_SECRET").expect("Please set ACCESS_SECRET");
    let consumer_token = egg_mode::KeyPair::new(c_key, c_secret);
    let access_token = egg_mode::KeyPair::new(a_key, a_secret);
    let token = egg_mode::Token::Access {
        consumer: consumer_token,
        access: access_token,
    };

    let user = egg_mode::auth::verify_tokens(&token).await.unwrap();

    println!("{}", user.screen_name);
}
