use actix_web::dev::ServiceRequest;
use actix_web::Error;
use sha2::{Digest, Sha256};
use actix_web_httpauth::extractors::basic::BasicAuth;
use std::env;

fn get_hash(password: &String, salt: &String) -> String {
    let mut hasher = Sha256::new();
    let salted_password = format!("{}/{}", password, salt);
    hasher.update(salted_password);
    let hashed: String = format!("{:X}", hasher.finalize());
    hashed
}

fn auth_error() -> Error {
    let err = std::io::Error::new(std::io::ErrorKind::Other, "Unauthorized");
    Error::from(err)
}

#[derive(Debug, Clone)]
pub struct EnvAuth {
    username: String,
    password: String,
    salt: String,
}

impl EnvAuth {
    pub fn new() -> Self {
        EnvAuth {
            username: env::var("AUTH_USERNAME").expect("Missing AUTH_USERNAME env variable"),
            password: env::var("AUTH_PASSWORD").expect("Missing AUTH_PASSWORD env variable"),
            salt: env::var("AUTH_SALT").expect("Missing AUTH_SALT env variable"),
        }
    }

    pub async fn check_auth(
        self,
        req: ServiceRequest,
        credentials: BasicAuth,
    ) -> Result<ServiceRequest, Error> {
        let hashed_password = get_hash(&self.password, &self.salt);
        let is_correct = credentials.password()
            .map(|password| {
                let curr_password = get_hash(&password.as_ref().to_string(), &self.salt);
                curr_password == hashed_password
            })
            .unwrap_or(false);

        if is_correct {
            Ok(req)
        } else {
            Err(auth_error())
        }
    }

    pub fn generate_signature(
        self,
        link: String,
    ) -> String {
        get_hash(&format!("{}/{}", link, self.password), &self.salt)
    }

    pub fn check_signature(
        self,
        link: String,
        signature: String
    ) -> bool {
        self.generate_signature(link) == signature
    }
}
