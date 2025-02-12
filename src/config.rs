use figment::{
    value::{Dict, Map},
    Error, Metadata, Profile, Provider,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub db_url: String,
    pub addr: String,
    pub port: u16,
    pub log_level: String,
    pub jwt_secret: String,
    pub access_token_exp_time_minutes: i64,
    pub refresh_token_exp_time_days: i64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            db_url: "postgres://postgres:postgres@mergeminds-postgres:5432/mergeminds".to_string(),
            addr: "0.0.0.0".to_string(),
            port: 8000,
            log_level: "info".to_string(),
            jwt_secret: "secret".to_string(),
            access_token_exp_time_minutes: 30,
            refresh_token_exp_time_days: 30,
        }
    }
}

impl Provider for Config {
    fn metadata(&self) -> Metadata {
        Metadata::named("MM backend config")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        figment::providers::Serialized::defaults(Config::default()).data()
    }
}
