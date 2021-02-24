use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl LoginRequest {
    pub fn from_env() -> Self {
        Self {
            email: std::env::var("EMAIL").expect("Expected a EMAIL in the environment"),
            password: std::env::var("PASSWORD").expect("Expected a PASSWORD in the environment"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginReply {
    pub jwt: String,
}

#[derive(Serialize, Deserialize)]
pub struct AssociationRequest {
    pub discord_association_code: String,
    pub discord_id: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "association")]
#[serde(rename_all = "camelCase")]
pub enum UserType {
    Staff,
    Empresa,
    Participante,
    Orador,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorReply {
    pub error: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SafiraIdResponse {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BadgeGiveRequest {
    pub redeem: SmolBoyRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SmolBoyRequest {
    pub attendee_id: String,
    pub badge_id: u32,
}
