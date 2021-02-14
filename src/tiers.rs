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
    sync::Mutex,
};

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
    spotlight: ChannelId,
}

impl Default for Tiers {
    fn default() -> Self {
        Self {
            tiers: HashMap::new(),
            spotlight: ChannelId::default(),
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

    pub fn tier(&self, name: &str) -> Option<&Tier> {
        self.tiers.get(name)
    }

    pub fn put(&mut self, name: String, tier: Tier) {
        self.tiers.insert(name, tier);
        self.save();
    }

    pub fn rm(&mut self, to_rm: &str) -> Option<String> {
        let rm = self.tiers.remove(to_rm);
        self.save();
        if let Some(_) = rm {
            Some(to_rm.to_owned())
        } else {
            None
        }
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
}
