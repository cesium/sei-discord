use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginReply {
    jwt: String,
}

#[derive(Serialize, Deserialize)]
pub struct AssociationRequest {
    discord_association_code: String,
    discord_id: String,
}

#[derive(Serialize, Deserialize)]
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
    error: String,
}
