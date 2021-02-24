pub mod company;
pub mod tier;

use crate::rocket::futures::StreamExt;
use crate::tiers::{Guild, TIERS};
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::{
        channel::Message,
        id::{ChannelId, UserId},
    },
    prelude::*,
};
#[group]
#[commands(spotlight_set, news_set, broadcast, say, give_badge, give_badge_all)]
struct Commands;

#[command]
#[min_args(1)]
#[required_permissions(ADMINISTRATOR)]
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
#[required_permissions(ADMINISTRATOR)]
pub async fn news_set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let news_channel = args.single::<ChannelId>()?;
    let mut locked_tiers = TIERS.lock().await;
    locked_tiers
        .0
        .entry(msg.guild_id.unwrap())
        .or_insert_with(Guild::default)
        .news_channel = Some(news_channel);
    locked_tiers.save()?;
    msg.reply(&ctx, format!("News channel set to {}", news_channel))
        .await?;
    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
pub async fn broadcast(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    for channel in msg.guild_id.unwrap().channels(&ctx).await?.values() {
        if channel.say(&ctx, args.rest()).await.is_err() {};
    }
    Ok(())
}

#[command]
#[min_args(2)]
#[required_permissions(MANAGE_ROLES)]
pub async fn say(ctx: &Context, _msg: &Message, mut args: Args) -> CommandResult {
    let channel_id = args.single::<ChannelId>()?;
    channel_id.say(&ctx.http, args.rest()).await?;
    Ok(())
}

#[command]
#[min_args(2)]
#[required_permissions(MANAGE_ROLES)]
pub async fn give_badge(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_id = args.single::<UserId>()?;
    let badge = args.single::<u32>()?;
    if crate::handler::give_badge(user_id, badge).await.is_some() {
        msg.reply(&ctx.http, "Badge given with success").await?;
    } else {
        msg.reply(&ctx.http, "Badge attribution failed").await?;
    }
    Ok(())
}

#[command]
#[min_args(1)]
#[required_permissions(ADMINISTRATOR)]
pub async fn give_badge_all(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let badge = args.single::<u32>()?;
    let mut iter = msg.guild_id.unwrap().members_iter(&ctx.http).boxed();
    while let Some(user) = iter.next().await {
        if let Ok(user) = user {
            crate::handler::give_badge(user.user.id, badge).await;
        }
    }

    msg.reply(&ctx.http, "Badges given with some success")
        .await?;
    Ok(())
}
