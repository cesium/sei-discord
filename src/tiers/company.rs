use serde::{Deserialize, Serialize};
use serenity::{
    http::CacheHttp,
    model::{
        channel::ChannelType,
        id::{ChannelId, GuildId, RoleId, UserId},
    },
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Company {
    pub guild_id: GuildId,
    role_id: RoleId,
    cat_id: ChannelId,
    spotlight: bool,
    default_voice: ChannelId,
    channels: Vec<ChannelId>,
    users: Vec<UserId>,
}

impl Company {
    pub async fn create(name: &str, ctx: &impl CacheHttp, gid: GuildId) -> serenity::Result<Self> {
        let role_name = name.to_lowercase().replace("\"", "");
        let upper_name = role_name.replace(" ", "-");
        let role = gid
            .create_role(&ctx.http(), |z| {
                z.hoist(true).mentionable(true).name(&role_name)
            })
            .await?;
        let category = gid
            .create_channel(&ctx.http(), |c| {
                c.name(&upper_name).kind(ChannelType::Category)
            })
            .await?;
        let text = gid
            .create_channel(&ctx.http(), |c| {
                c.name(format!("{}-text", &upper_name))
                    .kind(ChannelType::Text)
                    .category(category.id)
            })
            .await?;
        let voice = gid
            .create_channel(&ctx.http(), |c| {
                c.name(format!("{}-voice", &upper_name))
                    .kind(ChannelType::Voice)
                    .category(category.id)
            })
            .await?;
        Ok(Company {
            guild_id: gid,
            role_id: role.id,
            cat_id: category.id,
            default_voice: voice.id,
            spotlight: false,
            channels: vec![voice.id, text.id],
            users: Vec::new(),
        })
    }

    pub async fn delete(&self, ctx: &impl CacheHttp) -> serenity::Result<()> {
        self.guild_id.delete_role(&ctx.http(), self.role_id).await?;
        self.cat_id.delete(&ctx.http()).await?;
        for channel in &self.channels {
            channel.delete(&ctx.http()).await?;
        }
        Ok(())
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

    pub async fn spotlight_start(
        &mut self,
        ctx: &impl CacheHttp,
        spotlight: ChannelId,
    ) -> serenity::Result<()> {
        for channel in &self.channels {
            channel.edit(&ctx.http(), |c| c.category(spotlight)).await?;
        }
        self.spotlight = true;
        Ok(())
    }

    pub async fn spotlight_end(&mut self, ctx: &impl CacheHttp) -> serenity::Result<()> {
        for channel in &self.channels {
            channel
                .edit(&ctx.http(), |c| c.category(self.cat_id))
                .await?;
        }
        self.spotlight = false;
        Ok(())
    }

    pub fn addch(&mut self, channel: ChannelId) {
        self.channels.push(channel);
    }
    pub fn rmch(&mut self, channel: ChannelId) {
        if let Some(pos) = self.channels.iter().position(|x| *x == channel) {
            self.channels.remove(pos);
        }
    }
}
