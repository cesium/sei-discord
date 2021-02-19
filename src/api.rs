mod types;
use crate::tiers::TIERS;
use rocket::State;
use serenity::CacheAndHttp;
use std::sync::Arc;
use types::ApiKey;

#[post("/spotlight", data = "<company_name>")]
pub async fn spotlight_start(
    _wakey: ApiKey,
    company_name: String,
    discord: State<'_, Arc<CacheAndHttp>>,
) -> Option<()> {
    let mut locked_tier = TIERS.lock().await;
    for (_guild_id, guild) in locked_tier.0.iter_mut() {
        let company_name = company_name
            .to_lowercase()
            .replace("\"", "")
            .replace(" ", "-");
        let spot = guild.spotlight;
        let company = guild
            .flat_iter()
            .find(|(k, _v)| **k == company_name)
            .map(|(_k, v)| v);
        if let Some(company) = company {
            if let Some(spot) = spot {
                if company.spotlight_start(&**discord, spot).await.is_ok() {
                    guild.spotlight_company = Some(company_name.to_owned());
                } else {
                }
                break;
            }
        } else {
        }
    }
    locked_tier.save();
    Some(())
}

#[delete("/spotlight")]
pub async fn spotlight_end(_wakey: ApiKey, discord: State<'_, Arc<CacheAndHttp>>) -> Option<()> {
    let mut locked_tier = TIERS.lock().await;
    for (_guild_id, guild) in locked_tier.0.iter_mut() {
        if let Some(company_name) = guild.spotlight_company.clone() {
            let company = guild
                .flat_iter()
                .find(|(k, _v)| **k == company_name)
                .map(|(_k, v)| v);
            if let Some(company) = company {
                if company.spotlight_end(&**discord).await.is_ok() {
                    guild.spotlight_company = None;
                } else {
                }
                break;
            } else {
            }
        }
    }
    locked_tier.save();
    Some(())
}
