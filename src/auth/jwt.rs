use crate::config::Config;

use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub enum TokenType {
    AccessToken,
    RefreshToken,
}

pub fn create_token(
    config: &Config,
    email: &str,
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
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
}

pub fn create_tokens(
    config: &Config,
    email: &str,
) -> jsonwebtoken::errors::Result<(String, String)> {
    let access_token = create_token(config, email, TokenType::AccessToken)?;
    let refresh_token = create_token(config, email, TokenType::RefreshToken)?;

    Ok((access_token, refresh_token))
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
