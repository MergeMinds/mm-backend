use sqlx::postgres::PgPoolOptions;

#[derive(Clone)]
pub struct PgConnection {
    pub(crate) pool: sqlx::Pool<sqlx::Postgres>,
}

impl PgConnection {
    pub async fn new(db_url: &str) -> sqlx::Result<PgConnection> {
        Ok(PgConnection {
            pool: PgPoolOptions::new().connect(db_url).await?,
        })
    }
}
