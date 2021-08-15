use dotenv::dotenv;
use egg_mode;
use egg_mode::tweet::DraftTweet;
use std::env;
use std::{thread, time};
use tokio::prelude::*;

pub struct Timeline {
    pub count: i32,
    pub max_id: Option<u64>,
    pub min_id: Option<u64>,
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
    println!("**************************************************************");

    // get timeline
    let timeline = egg_mode::tweet::home_timeline(&token).with_page_size(10);

    let (mut timeline, feed) = timeline.older(None).await.unwrap();

    // print timeline
    for tweet in &*feed {
        println!(
            "<@{}> {}",
            tweet.user.as_ref().unwrap().screen_name,
            tweet.text
        );
    }
    // reload the timeline with only what's new
    println!("**************************************************************");
    let delay = time::Duration::from_secs(3);
    loop {
        println!("sleeping for 3 sec");
        thread::sleep(delay);
        let timeline = egg_mode::tweet::home_timeline(&token).with_page_size(10);
        let (mut timeline, _feed) = timeline.start().await.unwrap();
        let reload_id = timeline.max_id.unwrap();
        //reload the timeline with only what's new
        timeline.reset();
        let (timeline, _new_posts) = timeline.older(Some(reload_id)).await.unwrap();
        // print timeline
        for tweet in &*_new_posts {
            println!(
                "<@{}> {}",
                tweet.user.as_ref().unwrap().screen_name,
                tweet.text
            );
        }
    }
}
