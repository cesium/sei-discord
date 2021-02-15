pub mod company;
pub mod tier;

use crate::tiers::TIERS;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::{channel::Message, id::ChannelId},
    prelude::*,
};
#[group]
#[commands(spotlight_set, news_set, broadcast)]
#[required_permissions(ADMINISTRATOR)]
struct Commands;

#[command]
#[min_args(1)]
pub async fn spotlight_set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let spotlight_cat = args.single::<ChannelId>()?;
    TIERS
        .lock()
        .await
        .spotlight
        .insert(msg.guild_id.unwrap(), spotlight_cat);
    TIERS.lock().await.save()?;
    msg.reply(&ctx, format!("Spotlight category set to {}", spotlight_cat))
        .await?;
    Ok(())
}

#[command]
#[min_args(1)]
pub async fn news_set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let news_channel = args.single::<ChannelId>()?;
    TIERS
        .lock()
        .await
        .spotlight
        .insert(msg.guild_id.unwrap(), news_channel);
    TIERS.lock().await.save()?;
    msg.reply(&ctx, format!("Spotlight category set to {}", news_channel))
        .await?;
    Ok(())
}

#[command]
pub async fn broadcast(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    for channel in msg.guild_id.unwrap().channels(&ctx).await?.values() {
        channel.say(&ctx, args.rest()).await;
    }
    Ok(())
}
