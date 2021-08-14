use dotenv::dotenv;
use egg_mode;
use egg_mode::tweet::DraftTweet;
use std::env;
use tokio::prelude::*;

pub struct Timeline {
    pub count: i32,
    pub max_id: Option<u64>,
    pub min_id: Option<u64>,
    // some fields omitted
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let c_key = env::var("CONSUMER_KEY").expect("Please set consumer-key in .env");
    let c_secret = env::var("CONSUMER_SECRET").expect("Please set consumer-secret in .env");
    let a_key = env::var("ACCESS_KEY").expect("Please set access-key in .env");
    let a_secret = env::var("ACCESS_SECRET").expect("Please set access-secret in .env");
    let consumer_token = egg_mode::KeyPair::new(c_key, c_secret);
    let access_token = egg_mode::KeyPair::new(a_key, a_secret);
    let token = egg_mode::Token::Access {
        consumer: consumer_token,
        access: access_token,
    };

    let user = egg_mode::auth::verify_tokens(&token).await.unwrap();
    println!("User @{}'s timeline", user.screen_name);

    let timeline = egg_mode::tweet::home_timeline(&token).with_page_size(10);
    let (timeline, feed) = timeline.start().await.unwrap();
    loop {
        let mut id: u64 ~
        for tweet in &*feed {
            println!(
                "<@{}> {}",
                tweet.user.as_ref().unwrap().screen_name,
                tweet.text
            );
        }
        // let (timeline, feed) = timeline.older(None).await.unwrap();
        // for tweet in &*feed {
        //     println!(
        //         "<@{}> {}",
        //         tweet.user.as_ref().unwrap().screen_name,
        //         tweet.text
        //     );
        // }
    }
}
