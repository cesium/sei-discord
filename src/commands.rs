pub mod company;
pub mod tier;

use crate::tiers::{Guild, TIERS};
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::{channel::Message, id::ChannelId},
    prelude::*,
};
#[group]
#[commands(spotlight_set, news_set, broadcast, say)]
#[required_permissions(ADMINISTRATOR)]
struct Commands;

#[command]
#[min_args(1)]
pub async fn spotlight_set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let spotlight_cat = args.single::<ChannelId>()?;
    let mut locked_tiers = TIERS.lock().await;
    locked_tiers
        .0
        .entry(msg.guild_id.unwrap())
        .or_insert_with(Guild::default)
        .spotlight = Some(spotlight_cat);
    locked_tiers.save()?;
    msg.reply(&ctx, format!("Spotlight category set to {}", spotlight_cat))
        .await?;
    Ok(())
}

#[command]
#[min_args(1)]
pub async fn news_set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let news_channel = args.single::<ChannelId>()?;
    let mut locked_tiers = TIERS.lock().await;
    locked_tiers
        .0
        .entry(msg.guild_id.unwrap())
        .or_insert_with(Guild::default)
        .news_channel = Some(news_channel);
    locked_tiers.save()?;
    msg.reply(&ctx, format!("Spotlight category set to {}", news_channel))
        .await?;
    Ok(())
}

#[command]
pub async fn broadcast(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    for channel in msg.guild_id.unwrap().channels(&ctx).await?.values() {
        if channel.say(&ctx, args.rest()).await.is_err() {};
    }
    Ok(())
}

#[command]
#[min_args(2)]
pub async fn say(ctx: &Context, _msg: &Message, mut args: Args) -> CommandResult {
    let channel_id = args.single::<ChannelId>()?;
    channel_id.say(&ctx.http, args.rest()).await?;
    Ok(())
}
