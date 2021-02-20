mod types;
use crate::tiers::TIERS;
use rocket::State;
use rocket_contrib::json::Json;
use serenity::CacheAndHttp;
use std::sync::Arc;
use types::{ApiKey, CompanyVCResponse, SpotlightReq};

#[post("/spotlight", format = "json", data = "<company_name>")]
pub async fn spotlight_start(
    _wakey: ApiKey,
    company_name: Json<SpotlightReq>,
    discord: State<'_, Arc<CacheAndHttp>>,
) -> Option<()> {
    let company_name = &company_name.company;
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
                if company.0.spotlight_start(&**discord, spot).await.is_ok() {
                    guild.spotlight_company = Some(company_name.to_owned());
                } else {
                }
                return Some(());
            }
        } else {
        }
    }
    if let Err(why) = locked_tier.save() {
        println!("Client error: {:?}", why);
    }
    None
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
                if company.0.spotlight_end(&**discord).await.is_ok() {
                    guild.spotlight_company = None;
                } else {
                }
                return Some(());
            } else {
            }
        }
    }
    if let Err(why) = locked_tier.save() {
        println!("Client error: {:?}", why);
    }
    None
}

#[get("/voice/<company_name>")]
pub async fn company_vc(
    _wakey: ApiKey,
    company_name: String,
    discord: State<'_, Arc<CacheAndHttp>>,
) -> Option<Json<CompanyVCResponse>> {
    let mut locked_tier = TIERS.lock().await;
    for (_guild_id, guild) in locked_tier.0.iter_mut() {
        let company_name = company_name
            .to_lowercase()
            .replace("\"", "")
            .replace(" ", "-");
        let company = guild
            .flat_iter()
            .find(|(k, _v)| **k == company_name)
            .map(|(_k, v)| v);
        if let Some(company) = company {
            return Some(Json(CompanyVCResponse {
                users: company
                    .0
                    .default_voice
                    .to_channel(&**discord)
                    .await
                    .unwrap()
                    .guild()
                    .unwrap()
                    .members(&*discord.cache)
                    .await
                    .unwrap()
                    .iter()
                    .map(|x| x.user.id)
                    .collect(),
            }));
        } else {
        }
    }
    None
}

pub fn main(arc: Arc<CacheAndHttp>) {
    tokio::spawn(async {
        if let Err(why) = rocket::ignite()
            .mount("/", routes![spotlight_start, spotlight_end, company_vc])
            .manage(arc)
            .launch()
            .await
        {
            println!("Rocket error: {:?}", why);
        };
    });
}
