use crate::config::CONFIG;
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SpotlightReq {
    pub company: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiKey(Option<String>);

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    async fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("token");
        if let Some(key) = &CONFIG.wakey {
            match token {
                Some(token) => {
                    // check validity
                    if key == token {
                        Outcome::Success(ApiKey(Some(token.to_string())))
                    } else {
                        Outcome::Failure((Status::Unauthorized, ()))
                    }
                }
                // token does not exist
                None => Outcome::Failure((Status::Unauthorized, ())),
            }
        } else {
            Outcome::Success(ApiKey(None))
        }
    }
}
