pub mod company;
use crate::config::CONFIG;
use company::Company;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serenity::{
    client::Context,
    model::id::{ChannelId, GuildId, RoleId, UserId},
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
pub struct Tiers {
    tiers: HashMap<String, Tier>,
    pub spotlight: HashMap<GuildId, ChannelId>,
    pub news_channel: HashMap<GuildId, ChannelId>,
}

impl Default for Tiers {
    fn default() -> Self {
        Self {
            tiers: HashMap::new(),
            spotlight: HashMap::new(),
            news_channel: HashMap::new(),
        }
    }
}

impl Tiers {
    fn save(&self) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&CONFIG.roles_location)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self)?;
        Ok(())
    }

    pub fn tier(&mut self, name: &str) -> Option<&mut Tier> {
        self.tiers.get_mut(name)
    }

    pub fn exists(&self, name: &str) -> bool {
        self.tiers.contains_key(name)
    }

    pub fn put(&mut self, name: String, tier: Tier) {
        self.tiers.insert(name, tier);
        self.save();
    }

    pub fn rm(&mut self, to_rm: &str) -> Option<Tier> {
        let rm = self.tiers.remove(to_rm);
        self.save();
        rm
    }

    pub fn flat_iter(&mut self) -> impl Iterator<Item = (&'_ String, &'_ mut Company)> {
        self.tiers
            .iter_mut()
            .flat_map(|(_k, v)| v.companies.iter_mut())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tier {
    guild_id: GuildId,
    role_id: RoleId,
    companies: HashMap<String, Company>,
}

impl Tier {
    pub async fn create(name: &str, ctx: &Context, gid: GuildId) -> serenity::Result<Self> {
        let upper_name = name.to_uppercase();
        let role = gid
            .create_role(&ctx.http, |z| {
                z.hoist(false).mentionable(true).name(&upper_name)
            })
            .await?;
        Ok(Tier {
            guild_id: gid,
            role_id: role.id,
            companies: HashMap::new(),
        })
    }

    pub async fn give(&self, ctx: &Context, user: UserId) -> serenity::Result<()> {
        match self.guild_id.member(&ctx, user).await {
            Ok(mut member) => {
                member.add_role(&ctx, self.role_id).await?;
                Ok(())
            }
            Err(a) => Err(a),
        }
    }

    pub async fn delete(&self, ctx: &Context) -> serenity::Result<()> {
        self.guild_id.delete_role(&ctx, self.role_id).await?;
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
}
