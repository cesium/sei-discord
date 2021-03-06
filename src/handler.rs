pub mod company;
use crate::{
    config::CONFIG,
    requests::*,
    tiers::{Guild, TIERS},
};
use async_once::AsyncOnce;
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
use uuid::Uuid;

lazy_static! {
    pub static ref JWT: AsyncOnce<Option<String>> = AsyncOnce::new(async {
        let login_request: LoginRequest = LoginRequest::from_env();
        match reqwest::Client::new()
            .post(
                reqwest::Url::parse(format!("{}/api/auth/sign_in", &CONFIG.backend_ip).as_str())
                    .unwrap(),
            )
            .json(&login_request)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    response.json::<LoginReply>().await.ok().map(|x| x.jwt)
                } else {
                    println!("{}", response.json::<ErrorReply>().await.unwrap().error);
                    None
                }
            }
            _ => {
                println!("Couldn't login on backend");
                None
            }
        }
    });
}

impl UserType {
    const fn as_role(&self) -> RoleId {
        match self {
            Self::Staff => RoleId(793534104339349534),
            Self::Empresa => RoleId(813053096158298122),
            Self::Orador => RoleId(793537618138234912),
            Self::Participante => RoleId(793536131458531389),
        }
    }
}
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
                    .description("Para poderes ter acesso a todo o evento, cola aqui o código de associação ao discord que recebeste no teu email")
                    .thumbnail("https://cdn.discordapp.com/attachments/425980901344935937/813437097561554944/icon-final.png")
                })
            })
            .await;
        match dm {
            Ok(msg) => println!("{} joined", msg.channel_id),
            Err(why) => println!("{}", why),
        }
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        if Message::is_private(&new_message) {
            let mut message = String::from("Código inválido, tenta de novo");
            if let Ok(mut member) = GUILD_ID.member(&ctx, new_message.author.id).await {
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
                    if Uuid::parse_str(&new_message.content).ok().is_some() {
                        if let Some(role) = request_role(request).await {
                            let role_id = UserType::as_role(&role);
                            let _ = member.add_role(&ctx, role_id).await;
                            if role == UserType::Empresa {
                                send_company_embed(&ctx, new_message.author, GUILD_ID).await;
                                return;
                            } else if role == UserType::Participante {
                                if let Some(bg) = CONFIG.acred_badge {
                                    give_badge(new_message.author.id, bg).await;
                                }
                            }
                            message = String::from(
                                "O teu id foi validado, vais agora ter acesso aos canais da SEI.",
                            );
                        }
                    }
                } else if member
                    .roles(&ctx)
                    .await
                    .filter(|x| x.iter().any(|z| z.id == UserType::Empresa.as_role()))
                    .is_some()
                {
                    if company::try_give_company(
                        &ctx,
                        GUILD_ID,
                        new_message.author.id,
                        new_message.content.trim(),
                    )
                    .await
                    {
                        message = String::from(
                            "O seu id foi validado, terá agora acesso aos canais da SEI.",
                        );
                    } else {
                        message = String::from("Emprsa não encontrada");
                    }
                }
                new_message
                    .author
                    .dm(&ctx, |m| m.content(message))
                    .await
                    .unwrap();
            }
        }
    }
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        println!("{:?}", JWT.get().await);
    }
}

async fn request_role(association_request: AssociationRequest) -> Option<UserType> {
    if let Some(jwt) = JWT.get().await {
        match reqwest::Client::new()
            .post(
                reqwest::Url::parse(format!("{}/api/v1/association", &CONFIG.backend_ip).as_str())
                    .unwrap(),
            )
            .bearer_auth(jwt)
            .json(&association_request)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    Some(response.json::<UserType>().await.unwrap())
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        None
    }
}

async fn send_company_embed(ctx: &Context, user: User, guild_id: GuildId) {
    let mut company_names = std::collections::BTreeMap::new();
    for (k, v) in TIERS
        .lock()
        .await
        .0
        .entry(guild_id)
        .or_insert_with(Guild::default)
        .no_iter()
    {
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
            .thumbnail("https://cdn.discordapp.com/attachments/425980901344935937/813437097561554944/icon-final.png")
            .fields(company_names.iter().map(|(k, v)| (k,v,true)))
        })
    })
    .await
        .unwrap();
}

pub async fn give_badge(uid: UserId, badge_id: u32) -> Option<()> {
    if let Some(jwt) = JWT.get().await {
        match reqwest::Client::new()
            .get(
                reqwest::Url::parse(
                    format!("{}/api/v1/association/{}", &CONFIG.backend_ip, uid).as_str(),
                )
                .unwrap(),
            )
            .bearer_auth(jwt)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    let attendee_id = response.json::<SafiraIdResponse>().await.unwrap().id;
                    let badge_req = BadgeGiveRequest {
                        redeem: SmolBoyRequest {
                            attendee_id,
                            badge_id,
                        },
                    };
                    match reqwest::Client::new()
                        .post(
                            reqwest::Url::parse(
                                format!("{}/api/v1/redeems", &CONFIG.backend_ip).as_str(),
                            )
                            .unwrap(),
                        )
                        .bearer_auth(jwt)
                        .json(&badge_req)
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if response.status().is_success() {
                                Some(())
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        None
    }
}
