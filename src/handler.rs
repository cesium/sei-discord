pub mod company;
use crate::{
    config::CONFIG,
    requests::{AssociationRequest, ErrorReply, LoginReply, LoginRequest, UserType},
    tiers::{TIERS},
};
use lazy_static::lazy_static;
use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
        guild::Member,
        prelude::{GuildId, RoleId, UserId},
        user::User,
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
            Self::Empresa => RoleId(813053096158298122),
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
            let mut member = GUILD_ID.member(&ctx, new_message.author.id).await.unwrap();
            if member
                .roles(&ctx)
                .await
                .filter(|x| {
                    !x.iter().any(|z| {
                        z.id == UserType::Empresa.as_role()
                            || z.id == UserType::Participante.as_role()
                    })
                })
                .is_some()
            {
                let request = AssociationRequest {
                    discord_association_code: new_message.content.to_owned(),
                    discord_id: new_message.author.id.to_string(),
                };
                if let Some(role) = Uuid::parse_str(&new_message.content)
                    .ok()
                    .and_then(|_| request_role(request))
                {
                    let role_id = UserType::as_role(&role);
                    let _ = member.add_role(&ctx, role_id).await;
                    if role == UserType::Empresa {
                        send_company_embed(&ctx, new_message.author, GUILD_ID).await;
                        return;
                    }
                    message = String::from(
                        "O teu id foi validado, vais agora ter acesso aos canais da SEI.",
                    );
                }
            } else if member
                .roles(&ctx)
                .await
                .filter(|x| x.iter().any(|z| z.id == UserType::Empresa.as_role()))
                .is_some()
            {
                if company::try_give_company(&ctx, GUILD_ID, new_message.author.id, new_message.content.trim()).await {
                    message = String::from(
                        "O seu id foi validado, terá agora acesso aos canais da SEI.",
                    );
                } else {
                    message = String::from(
                        "Emprsa não encontrada",
                    );
                }
            }
            new_message
                .author
                .dm(&ctx, |m| m.content(message))
                .await
                .unwrap();
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        send_company_embed(
            &ctx,
            UserId(193043741676797952).to_user(&ctx).await.unwrap(),
            GuildId(481544751158394880),
        )
        .await;
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

async fn send_company_embed(ctx: &Context, user: User, guild_id: GuildId) {
    let mut company_names = std::collections::HashMap::new();
    for (k, v) in TIERS.lock().await.0.get(&guild_id).unwrap().no_iter() {
        company_names.insert(
            k.to_owned(),
            v.company_names()
                .map(|x| x.to_owned())
                .collect::<Vec<String>>()
                .join("\n"),
        );
    }
    user.dm(&ctx, |m| { m.embed(|e| {
        e.title("Escolha a sua empresa")
            .description("Para concluir o processo de registo, responda a esta mensagem com a empresa a que pertence, da lista abaixo")
            .fields(company_names.iter().map(|(k, v)| (k,v,true))) 
        })
    })
    .await
        .unwrap();
}
