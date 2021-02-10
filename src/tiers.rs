use crate::config::CONFIG;
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
pub struct Tiers(HashMap<String, Tier>);

impl Default for Tiers {
    fn default() -> Self {
        Self(HashMap::new())
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tier {
    guild_id: GuildId,
    role_id: RoleId,
    companies: HashMap<String, Company>,
}

impl Tier {
    pub async fn give_tier(&self, ctx: Context, user: UserId) -> serenity::Result<()> {
        match self.guild_id.member(&ctx, user).await {
            Ok(mut member) => {
                member.add_role(&ctx, self.role_id).await?;
                Ok(())
            }
            Err(a) => Err(a),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Company {
    role_id: RoleId,
    channels: Vec<ChannelId>,
}
