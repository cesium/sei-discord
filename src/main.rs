pub mod commands;
pub mod config;
pub mod handler;
pub mod requests;
pub mod tiers;

use crate::commands::tier::TIER_GROUP;
use handler::Handler;
use serenity::{framework::StandardFramework, prelude::*};
use std::env;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(
            StandardFramework::new()
                .configure(|c| c.prefix("$"))
                .group(&TIER_GROUP),
        )
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
