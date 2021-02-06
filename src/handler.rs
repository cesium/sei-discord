use crate::config::CONFIG;
use crate::requests::{AssociationRequest, ErrorReply, LoginReply, LoginRequest};
use core::time::Duration;
use lazy_static::lazy_static;
use phf::phf_map;
use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
        guild::Member,
        prelude::{GuildId, RoleId},
    },
    prelude::*,
};
use std::{fs::File, io::BufReader};
lazy_static! {
    pub static ref JWT: String = {
        let login_request: LoginRequest = File::open(&CONFIG.credentials_location)
            .map(BufReader::new)
            .and_then(|x| {
                serde_json::from_reader(x)
                    .map_err(|x| std::io::Error::new(std::io::ErrorKind::Other, x))
            })
            .unwrap();
        match reqwest::blocking::Client::new()
            .post(reqwest::Url::parse(format!("{}/sign_in", &CONFIG.backend_ip).as_str()).unwrap())
            .json(&login_request)
            .send()
        {
            Ok(response) => {
                if response.status().is_success() {
                    response.json::<LoginReply>().unwrap().jwt
                } else {
                    panic!("{}", response.json::<ErrorReply>().unwrap().error)
                }
            }
            _ => panic!("Couldn't login on backend"),
        }
    };
}

static ROLES: phf::Map<&'static str, RoleId> = phf_map! {
    "staff" => RoleId(793534104339349534),
    "exclusive" => RoleId(793536901201264660),
    "gold" => RoleId(793536966548652033),
    "silver" => RoleId(793537012468023328),
    "orador" => RoleId(793537618138234912),
    "participante" => RoleId(793536131458531389),
};

const GUILD_ID: GuildId = GuildId(793533219865755648);
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, _guild_id: GuildId, new_member: Member) {
        let dm = new_member
            .user
            .dm(&ctx, |m| {
                m.embed(|e| {
                    e.title("Bem-vindo(a) a SEI'21!")
                    .description("Para poderes ter acesso a todo o evento, segue o link x e cola aqui o codigo que la encontras para finalizar a tua inscricao")
                })
            })
            .await;
        match dm {
            Ok(msg) => {
                let a = msg
                    .channel_id
                    .await_reply(&ctx)
                    .timeout(Duration::from_secs(3600))
                    .await;
                if let Some(ms) = a {
                    println!("{:#}", ms.content);
                }
                //now we send to backend ids
                //and then we recive
            }
            Err(why) => println!("Error when direct messaging user: {:?}", why),
        }
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        let _request = AssociationRequest {
            discord_association_code: new_message.content.to_owned(),
            discord_id: new_message.author.id.to_string(),
        };
        if Message::is_private(&new_message) {
            if new_message.content == "give me" {
                let role_id = ROLES
                    .get("orador")
                    .expect("Safira, do your job properly pls");
                let member = GUILD_ID.member(&ctx, new_message.author.id).await;
                match member {
                    Ok(mut m) => {
                        let _ = m.add_role(&ctx, role_id).await;
                    }
                    Err(e) => println!("{}", e),
                }
            }
            new_message
                .author
                .dm(&ctx, |m| {
                    m.content("Não faço ideia se o teu ID é válido ainda")
                })
                .await
                .unwrap();
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        //println!("{}", JWT.as_str());
        println!("{} is connected!", ready.user.name);
    }
}
