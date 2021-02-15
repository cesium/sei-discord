use crate::tiers::{Tier as T, Tiers, TIERS};
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

#[group]
#[commands(create, rm)]
#[required_permissions(ADMINISTRATOR)]
#[prefixes("tier")]
struct Tier;

#[command]
pub async fn create(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut args = args.raw();
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
