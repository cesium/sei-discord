use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

lazy_static! {
    pub static ref CONFIG: Config = Config::from_env();
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub backend_ip: String,
    pub wakey: Option<String>,
    pub roles_location: PathBuf,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            wakey: std::env::var("WAKEY").ok(),
            roles_location: PathBuf::from_str(
                &std::env::var("ROLE_FILE").expect("Expected role file dir on env"),
            )
            .unwrap(),
            backend_ip: std::env::var("BACKEND_IP").expect("Expected IP on env"),
        }
    }
}
