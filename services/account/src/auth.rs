use crate::{db::models::User, Env};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum AuthErr {
    EncodeTokenErr(jsonwebtoken::errors::Error),
    DecodeTokenErr(jsonwebtoken::errors::Error),
}

pub fn create_jwt<T: Auth>(auth: &T) -> Result<String, AuthErr> {
    let claims = auth.get_claims();

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(Env::get().jwt_secret.as_bytes()),
    )
    .map_err(AuthErr::EncodeTokenErr)?;

    Ok(token)
}

pub fn verify(token: String) -> Result<Claims, AuthErr> {
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(Env::get().jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(AuthErr::DecodeTokenErr)?;

    Ok(decoded.claims)
}

pub trait Auth {
    fn get_claims(&self) -> Claims;
}

impl Auth for User {
    fn get_claims(&self) -> Claims {
        let valid_until = Utc::now() + Duration::hours(12);
        Claims {
            sub: self.google_id.clone(),
            exp: usize::try_from(valid_until.timestamp())
                .unwrap_or_else(|err| panic!("failed convert timestamp: {}", err)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // Subject (whom token refers to)
    pub sub: String,
    // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub exp: usize,
}
