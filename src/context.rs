use crate::{config::Config, db::core::PgConnection};

#[derive(Clone)]
pub struct Context {
    pub config: Config,
    pub db: PgConnection,
}
