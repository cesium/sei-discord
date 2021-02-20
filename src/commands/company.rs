use crate::tiers::{company::Company as Corp, Guild, TIERS};
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
#[commands(create, rm, addch, rmch, spotlight, adduser, rmuser)]
#[required_permissions(ADMINISTRATOR)]
#[prefixes("company")]
struct Company;

#[command]
#[min_args(2)]
pub async fn create(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut args = args.raw();
    let tier_str = args.next().unwrap();
    let mut vec = Vec::new();
    let mut locked_tiers = TIERS.lock().await;
    if let Some(tier) = locked_tiers
        .0
        .entry(msg.guild_id.unwrap())
        .or_insert_with(Guild::default)
        .tier(tier_str)
    {
        for arg in args {
            if let Ok(company) = Corp::create(arg, &ctx, msg.guild_id.unwrap()).await {
                tier.put(arg.to_uppercase(), company);
                vec.push(arg);
            }
        }
    }
    locked_tiers.save()?;
    msg.reply(&ctx, format!("Created companies: {}", vec.join(" ")))
        .await?;
    Ok(())
}

#[command]
#[min_args(1)]
pub async fn rm(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut args = args.raw();
    let tier = args.next().unwrap();
    let mut vec = Vec::new();
    let mut locked_tiers = TIERS.lock().await;
    if let Some(tier) = locked_tiers
        .0
        .entry(msg.guild_id.unwrap())
        .or_insert_with(Guild::default)
        .tier(tier)
    {
        for arg in args {
            if let Some(company) = tier.rm(&arg.to_uppercase()) {
                company.delete(&ctx.http).await?;
                vec.push(arg);
            }
        }
    }
    locked_tiers.save()?;
    msg.reply(&ctx, format!("Removed companies: {}", vec.join(" ")))
        .await?;
    Ok(())
}

#[command]
#[min_args(2)]
pub async fn addch(_ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let company = args.single::<String>().unwrap().to_uppercase();
    let mut locked_tiers = TIERS.lock().await;
    if let Some(company) = locked_tiers
        .0
        .entry(msg.guild_id.unwrap())
        .or_insert_with(Guild::default)
        .flat_iter()
        .find(|(k, _v)| **k == company)
        .map(|(_k, v)| v)
    {
        while !args.is_empty() {
            company.0.addch(args.single::<ChannelId>()?);
        }
    };
    locked_tiers.save()?;
    Ok(())
}

#[command]
#[min_args(2)]
pub async fn rmch(_ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let company = args.single::<String>().unwrap().to_uppercase();
    let mut locked_tiers = TIERS.lock().await;
    if let Some(company) = locked_tiers
        .0
        .entry(msg.guild_id.unwrap())
        .or_insert_with(Guild::default)
        .flat_iter()
        .find(|(k, _v)| **k == company)
        .map(|(_k, v)| v)
    {
        while !args.is_empty() {
            company.0.rmch(args.single::<ChannelId>()?);
        }
    };
    locked_tiers.save()?;
    Ok(())
}

#[command]
#[min_args(2)]
pub async fn spotlight(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let company_name = args.single::<String>().unwrap().to_uppercase();
    let status = args.single::<bool>().unwrap();
    let mut locked_tiers = TIERS.lock().await;
    let locked_tier = locked_tiers
        .0
        .entry(msg.guild_id.unwrap())
        .or_insert_with(Guild::default);
    if let Some(spot) = locked_tier.spotlight {
        if let Some(company) = locked_tier
            .flat_iter()
            .find(|(k, _v)| **k == company_name)
            .map(|(_k, v)| v)
        {
            if status {
                if company.0.spotlight_start(&ctx.http, spot).await.is_ok() {
                    msg.reply(&ctx, format!("Spotlight enabled for {}", company_name))
                        .await?;
                } else {
                    msg.reply(
                        &ctx,
                        format!("Spotlight enabling failed for {}", company_name),
                    )
                    .await?;
                }
            } else if company.0.spotlight_end(&ctx.http).await.is_ok() {
                msg.reply(&ctx, format!("Spotlight disabled for {}", company_name))
                    .await?;
            } else {
                msg.reply(
                    &ctx,
                    format!("Spotlight disabling failed for {}", company_name),
                )
                .await?;
            }
        } else {
            msg.reply(&ctx, format!("Company {} doesnt exist", company_name))
                .await?;
        }
    } else {
        msg.reply(&ctx, "No spotlight category set").await?;
    }
    locked_tiers.save()?;
    Ok(())
}

#[command]
#[min_args(2)]
pub async fn adduser(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let company_name = args.single::<String>().unwrap().to_uppercase();
    let mut locked_tiers = TIERS.lock().await;
    if let Some(company) = locked_tiers
        .0
        .entry(msg.guild_id.unwrap())
        .or_insert_with(Guild::default)
        .flat_iter()
        .find(|(k, _v)| **k == company_name)
        .map(|(_k, v)| v)
    {
        let uid = args.single::<UserId>()?;
        msg.guild_id
            .unwrap()
            .member(&ctx, uid)
            .await?
            .add_role(&ctx, company.1)
            .await?;
        company.0.give(&*ctx, uid).await?;
        msg.reply(&ctx, format!("User added to {}", company_name))
            .await?;
    } else {
        msg.reply(&ctx, format!("Company {} not found", company_name))
            .await?;
    };
    Ok(())
}

#[command]
#[min_args(2)]
pub async fn rmuser(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let company_name = args.single::<String>().unwrap().to_uppercase();
    let mut locked_tiers = TIERS.lock().await;
    if let Some(company) = locked_tiers
        .0
        .entry(msg.guild_id.unwrap())
        .or_insert_with(Guild::default)
        .flat_iter()
        .find(|(k, _v)| **k == company_name)
        .map(|(_k, v)| v)
    {
        let uid = args.single::<UserId>()?;
        msg.guild_id
            .unwrap()
            .member(&ctx, uid)
            .await?
            .remove_role(&ctx, company.1)
            .await?;
        company.0.rmuser(&*ctx, uid).await?;
        msg.reply(&ctx, format!("User removed from {}", company_name))
            .await?;
    } else {
        msg.reply(&ctx, format!("Company {} not found", company_name))
            .await?;
    };
    Ok(())
}
