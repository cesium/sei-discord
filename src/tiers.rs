pub mod company;
use crate::config::CONFIG;
use company::Company;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serenity::{
    http::CacheHttp,
    model::{
        guild::Member,
        id::{ChannelId, GuildId, RoleId, UserId},
    },
};
use std::collections::HashMap;
use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter},
};
use tokio::sync::Mutex;

lazy_static! {
    pub static ref TIERS: Mutex<Tiers> = {
        Mutex::new(
            File::open(&CONFIG.roles_location)
                .map(BufReader::new)
                .and_then(|x| {
                    serde_json::from_reader(x)
                        .map_err(|x| std::io::Error::new(std::io::ErrorKind::Other, x))
                })
                .unwrap_or_default(),
        )
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tiers(pub HashMap<GuildId, Guild>);

impl Default for Tiers {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Guild {
    pub tiers: HashMap<String, Tier>,
    pub spotlight: Option<ChannelId>,
    pub news_channel: Option<ChannelId>,
    pub spotlight_company: Option<String>,
}

impl Default for Guild {
    fn default() -> Self {
        Self {
            tiers: HashMap::new(),
            spotlight: Option::None,
            spotlight_company: Option::None,
            news_channel: Option::None,
        }
    }
}

impl Tiers {
    pub fn save(&self) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&CONFIG.roles_location)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self)?;
        Ok(())
    }
}

impl Guild {
    pub fn tier(&mut self, name: &str) -> Option<&mut Tier> {
        self.tiers.get_mut(name)
    }

    pub fn exists(&self, name: &str) -> bool {
        self.tiers.contains_key(name)
    }

    pub fn put(&mut self, name: String, tier: Tier) {
        self.tiers.insert(name, tier);
    }

    pub fn rm(&mut self, to_rm: &str) -> Option<Tier> {
        self.tiers.remove(to_rm)
    }

    pub fn flat_iter(&mut self) -> impl Iterator<Item = (&'_ String, (&'_ mut Company, RoleId))> {
        self.tiers.iter_mut().flat_map(|(_k, v)| {
            let role_id = v.role_id;
            v.companies.iter_mut().map(move |(x, y)| (x, (y, role_id)))
        })
    }
    pub fn iter(&mut self) -> impl Iterator<Item = (&'_ String, &'_ mut Tier)> {
        self.tiers.iter_mut()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tier {
    guild_id: GuildId,
    role_id: RoleId,
    companies: HashMap<String, Company>,
}

impl Tier {
    pub async fn create(name: &str, ctx: &impl CacheHttp, gid: GuildId) -> serenity::Result<Self> {
        let upper_name = name.to_lowercase().replace("\"", "").replace(" ", "-");
        let role = gid
            .create_role(&ctx.http(), |z| {
                z.hoist(false).mentionable(false).name(&upper_name)
            })
            .await?;
        Ok(Tier {
            guild_id: gid,
            role_id: role.id,
            companies: HashMap::new(),
        })
    }

    pub async fn give(&self, ctx: &impl CacheHttp, user: UserId) -> serenity::Result<()> {
        match self.guild_id.member(&ctx, user).await {
            Ok(mut member) => {
                member.add_role(&ctx.http(), self.role_id).await?;
                Ok(())
            }
            Err(a) => Err(a),
        }
    }

    pub async fn rmuser(&self, ctx: &impl CacheHttp, user: UserId) -> serenity::Result<()> {
        match self.guild_id.member(&ctx, user).await {
            Ok(mut member) => {
                member.remove_role(&ctx.http(), self.role_id).await?;
                Ok(())
            }
            Err(a) => Err(a),
        }
    }

    pub async fn delete(&self, ctx: &impl CacheHttp) -> serenity::Result<()> {
        self.guild_id.delete_role(&ctx.http(), self.role_id).await?;
        for company in self.companies.values() {
            company.delete(&ctx).await?;
        }
        Ok(())
    }

    pub fn company(&self, name: &str) -> Option<&Company> {
        self.companies.get(name)
    }

    pub fn put(&mut self, name: String, company: Company) {
        self.companies.insert(name, company);
    }

    pub fn rm(&mut self, to_rm: &str) -> Option<Company> {
        self.companies.remove(to_rm)
    }

    pub fn exists(&self, name: &str) -> bool {
        self.companies.contains_key(name)
    }

    pub async fn give_company(
        &self,
        company_name: &str,
        ctx: &impl CacheHttp,
        user: UserId,
    ) -> serenity::Result<()> {
        self.give(&ctx, user).await?;
        if let Some(company) = self.companies.get(company_name) {
            company.give(&ctx, user).await
        } else {
            Err(serenity::prelude::SerenityError::Other("Not found"))
        }
    }
}
