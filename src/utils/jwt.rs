use crate::{config::Config, models::UserRole};

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

pub enum TokenType {
    AccessToken,
    RefreshToken,
}

pub fn create_token(
    config: &Config,
    email: &str,
    role: UserRole,
    token_type: TokenType,
) -> jsonwebtoken::errors::Result<String> {
    let duration = match token_type {
        TokenType::AccessToken => {
            chrono::Duration::minutes(config.access_token_exp_time_minutes)
        }
        TokenType::RefreshToken => {
            chrono::Duration::days(config.refresh_token_exp_time_days)
        }
    };
    let expiration = chrono::Utc::now()
        .checked_add_signed(duration)
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
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
}

pub fn validate_token(
    config: &Config,
    token: &str,
) -> jsonwebtoken::errors::Result<Claims> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 5;

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
}
