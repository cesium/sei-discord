use crate::tiers::{company::Company, Tier as T, TIERS};
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
#[prefixes("company")]
struct Tier;

#[command]
pub async fn create(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut args = args.raw();
    let tier_str = args.next().unwrap();
    let mut vec = Vec::new();
    if let Some(tier) = TIERS.lock().await.tier(tier_str) {
        for arg in args {
            if let Ok(company) = Company::create(arg, &ctx, msg.guild_id.unwrap()).await {
                tier.put(arg.to_owned(), company);
                vec.push(arg);
            }
        }
    }
    msg.reply(
        &ctx,
        format!("Foram criadas as empresas: {}", vec.join(" ")),
    )
    .await?;
    Ok(())
}

#[command]
pub async fn rm(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut args = args.raw();
    let tier = args.next().unwrap();
    let mut vec = Vec::new();
    if let Some(tier) = TIERS.lock().await.tier(tier) {
        for arg in args {
            if let Some(company) = tier.rm(arg) {
                company.delete(&ctx).await?;
                vec.push(arg);
            }
        }
    }
    msg.reply(
        &ctx,
        format!("Foram criadas as empresas: {}", vec.join(" ")),
    )
    .await?;
    Ok(())
}
