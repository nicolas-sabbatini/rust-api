use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use rocket::{
    async_trait,
    http::Status,
    request::{FromRequest, Outcome},
    response::status,
    Request,
};

pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    fn from_authotization_header(header: &str) -> Result<BasicAuth> {
        let split_header = header.split_whitespace().collect::<Vec<&str>>();
        if split_header.len() < 2 || split_header[0] != "Basic" {
            return Err(anyhow::anyhow!("Invalid Authorization header"));
        }

        Self::from_base64(split_header[1])
    }

    fn from_base64(encoded_base64: &str) -> Result<BasicAuth> {
        let decoded = general_purpose::URL_SAFE.decode(encoded_base64)?;
        let decoded = String::from_utf8(decoded)?;
        let split = decoded.split(':').collect::<Vec<&str>>();

        if split.len() < 2 {
            return Err(anyhow::anyhow!("Invalid Authorization header"));
        }

        Ok(BasicAuth {
            username: split[0].to_string(),
            password: split[1].to_string(),
        })
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let headers = request.headers();
        let auth_header = headers.get_one("Authorization");

        if let Some(auth_header) = auth_header {
            if let Ok(auth) = BasicAuth::from_authotization_header(auth_header) {
                // TODO: Check if user exists in database and if password is correct
                if auth.username != "foo" || auth.password != "bar" {
                    return Outcome::Failure((Status::Unauthorized, ()));
                }
                return Outcome::Success(auth);
            }
        }
        Outcome::Failure((Status::Unauthorized, ()))
    }
}
