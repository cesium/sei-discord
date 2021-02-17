use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SpotlightReq {
    pub company: String,
    pub status: bool,
}
