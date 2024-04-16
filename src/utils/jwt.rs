use crate::models::UserRole;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub role: UserRole,
}

pub fn create_token(
    email: &str,
    role: UserRole,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        // TODO(granatam): get expiration time from env and set it based on
        // token type
        .checked_add_signed(chrono::Duration::hours(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: email.to_owned(),
        exp: expiration as usize,
        role,
    };

    encode(
        &Header::default(),
        &claims,
        // TODO(granatam): get JWT secret from env
        &EncodingKey::from_secret("secret".as_ref()),
    )
}

pub fn validate_token(token: &str) -> jsonwebtoken::errors::Result<Claims> {
    decode::<Claims>(
        token,
        // TODO(granatam): get JWT secret from env
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
}
