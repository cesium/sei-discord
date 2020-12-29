use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, guild::Member, prelude::GuildId},
    prelude::*,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, _guild_id: GuildId, new_member: Member) {
        let dm = new_member
            .user
            .dm(&ctx, |m| {
                m.embed(|e| {
                    e.title("Bem-vindo(a) a SEI'21!");
                    e.description("Para poderes ter acesso a todo o evento, segue o link x e cola aqui o codigo que la encontras para finalizar a tua inscricao");
                    e
                });

                m
            })
            .await;

        if let Err(why) = dm {
            println!("Error when direct messaging user: {:?}", why);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
