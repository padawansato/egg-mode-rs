use dotenv::dotenv;
use egg_mode;
use std::env;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let c_key = env::var("CONSUMER_KEY").expect(".env/consumer-key");
    let c_secret = env::var("CONSUMER_SECRET").expect(".env/consumer-secret");
    let a_key = env::var("ACCESS_KEY").expect(".env/access-key");
    let a_secret = env::var("ACCESS_SECRET").expect(".env/access-secret");
    let consumer_token = egg_mode::KeyPair::new(c_key, c_secret);
    let access_token = egg_mode::KeyPair::new(a_key, a_secret);
    let token = egg_mode::Token::Access {
        consumer: consumer_token,
        access: access_token,
    };

    let user = egg_mode::auth::verify_tokens(&token).await.unwrap();

    println!("{}", user.screen_name);
}
