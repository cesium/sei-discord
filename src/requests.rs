use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
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
