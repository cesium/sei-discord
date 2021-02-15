pub mod commands;
pub mod config;
pub mod handler;
pub mod requests;
pub mod tiers;

use crate::commands::{company::COMPANY_GROUP, tier::TIER_GROUP, COMMANDS_GROUP};
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
                .group(&TIER_GROUP)
                .group(&COMPANY_GROUP)
                .group(&COMMANDS_GROUP),
        )
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
