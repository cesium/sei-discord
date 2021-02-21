use crate::tiers::{Guild, TIERS};
use serenity::{
    model::prelude::{GuildId, UserId},
    prelude::*,
};

pub async fn try_give_company(
    ctx: &Context,
    guild_id: GuildId,
    user_id: UserId,
    company_name: &str,
) -> bool {
    for (_name, tier) in TIERS
        .lock()
        .await
        .0
        .entry(guild_id)
        .or_insert_with(Guild::default)
        .no_iter()
    {
        if let Ok(Some(_)) = tier.give_company(company_name, &ctx, user_id).await {
            return true;
        }
    }
    false
}
