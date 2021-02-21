use crate::{
    config::CONFIG,
    requests::{AssociationRequest, ErrorReply, LoginReply, LoginRequest, UserType},
};
use lazy_static::lazy_static;
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
use uuid::Uuid;

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

impl UserType {
    fn as_role(&self) -> RoleId {
        match self {
            Self::Staff => RoleId(793534104339349534),
            Self::Empresa => RoleId(793536901201264660), // errado , mudar empresa
            Self::Orador => RoleId(793537618138234912),
            Self::Participante => RoleId(793536131458531389),
        }
    }
}
const GUILD_ID: GuildId = GuildId(769885792038289445);
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, _guild_id: GuildId, new_member: Member) {
        new_member
            .user
            .dm(&ctx, |m| {
                m.embed(|e| {
                    e.title("Bem-vindo(a) a SEI'21!")
                    .description("Para poderes ter acesso a todo o evento, segue o link x e cola aqui o codigo que la encontras para finalizar a tua inscricao")
                })
            })
            .await.unwrap();
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        let mut message = String::from("Código inválido, tenta de novo");
        if Message::is_private(&new_message) {
            let request = AssociationRequest {
                discord_association_code: new_message.content.to_owned(),
                discord_id: new_message.author.id.to_string(),
            };
            if let Some(role) = Uuid::parse_str(&new_message.content)
                .ok()
                .and_then(|_| request_role(request))
            {
                let role_id = UserType::as_role(&role);
                let member = GUILD_ID.member(&ctx, new_message.author.id).await;
                match member {
                    Ok(mut m) => {
                        let _ = m.add_role(&ctx, role_id).await;
                        message = String::from(
                            "O teu id foi validado, vais agora ter acesso aos canais da SEI.",
                        );
                    }
                    Err(e) => println!("{}", e),
                }
            }
        }
        new_message
            .author
            .dm(&ctx, |m| m.content(message))
            .await
            .unwrap();
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn request_role(association_request: AssociationRequest) -> Option<UserType> {
    match reqwest::blocking::Client::new()
        .post(reqwest::Url::parse(format!("{}/association", &CONFIG.backend_ip).as_str()).unwrap())
        .json(&association_request)
        .send()
    {
        Ok(response) => {
            if response.status().is_success() {
                Some(response.json::<UserType>().unwrap())
            } else {
                None
            }
        }
        _ => None,
    }
}
