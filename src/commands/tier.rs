use crate::tiers::{Tier as T, TIERS};
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::{channel::Message, id::UserId},
    prelude::*,
};

#[group]
#[commands(create, rm)]
#[required_permissions(ADMINISTRATOR)]
#[prefixes("tier")]
struct Tier;

#[command]
pub async fn create(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let args = args.raw();
    let mut vec = Vec::new();
    for arg in args {
        if let Ok(tier) = T::create(arg, &ctx, msg.guild_id.unwrap()).await {
            TIERS.lock().await.put(arg.to_owned(), tier);
            vec.push(arg);
        }
    }
    msg.reply(&ctx, format!("Foram criados os tiers: {}", vec.join(" ")))
        .await?;
    Ok(())
}

#[command]
pub async fn rm(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let args = args.raw();
    let mut vec = Vec::new();
    for arg in args {
        if let Some(s) = TIERS.lock().await.rm(arg) {
            s.delete(&ctx).await?;
            vec.push(arg);
        }
    }
    msg.reply(&ctx, format!("Foram removidos os tiers: {}", vec.join(" ")))
        .await?;
    Ok(())
}

#[command]
#[min_args(2)]
pub async fn adduser(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let tier_name = args.single::<String>().unwrap().to_uppercase();
    if let Some(tier) = TIERS.lock().await.tier(&tier_name) {
        let uid = args.single::<UserId>()?;
        tier.give(&ctx, uid).await?;
        msg.reply(&ctx, format!("User added to {}", tier_name))
            .await?;
    } else {
        msg.reply(&ctx, format!("Tier {} not found", tier_name))
            .await?;
    };
    Ok(())
}

pub async fn rmuser(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let tier_name = args.single::<String>().unwrap().to_uppercase();
    if let Some(tier) = TIERS.lock().await.tier(&tier_name) {
        let uid = args.single::<UserId>()?;
        tier.rmuser(&ctx, uid).await?;
        msg.reply(&ctx, format!("User removed from {}", tier_name))
            .await?;
    } else {
        msg.reply(&ctx, format!("Tier {} not found", tier_name))
            .await?;
    };
    Ok(())
}
