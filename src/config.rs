use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs::File, io::BufReader};

static CONFIG_PATH: &str = "config.json";

lazy_static! {
    pub static ref CONFIG: Config = {
        File::open(CONFIG_PATH)
            .map(BufReader::new)
            .and_then(|x| {
                serde_json::from_reader(x)
                    .map_err(|x| std::io::Error::new(std::io::ErrorKind::Other, x))
            })
            .unwrap()
    };
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub backend_ip: String,
    pub wakey: Option<String>,
    pub credentials_location: PathBuf,
    pub roles_location: PathBuf,
}
