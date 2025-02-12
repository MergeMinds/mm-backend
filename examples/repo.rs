use std::future::Future;

use futures::TryStreamExt;
use sqlx::{postgres::PgPoolOptions, Execute};

pub trait Entity {
    type Column: AsRef<str>;
    fn table_name() -> &'static str;
}

#[derive(Debug, strum::AsRefStr)]
pub enum UserColumn {
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "name")]
    Name,
    #[strum(serialize = "email")]
    Email,
    #[strum(serialize = "password")]
    Password,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl Entity for User {
    type Column = UserColumn;

    fn table_name() -> &'static str {
        "users"
    }
}

pub trait Repository<E: Entity> {
    type Error: std::error::Error + Send + Sync + 'static;
    type Database: sqlx::Database + Send;

    fn fetch_one<'a, V>(
        &'a self,
        key: E::Column,
        val: V,
    ) -> impl Future<Output = Result<Option<E>, Self::Error>> + Send + 'a
    where
        V: sqlx::Type<Self::Database> + sqlx::Encode<'a, Self::Database> + Send + 'a;
}

#[derive(Clone)]
pub struct UserRepo {
    conn: PgConnection,
}
impl UserRepo {
    pub fn new(conn: PgConnection) -> Self {
        Self { conn }
    }
}

impl Repository<User> for UserRepo {
    type Error = sqlx::Error;
    type Database = sqlx::Postgres;

    async fn fetch_one<'a, V>(
        &'a self,
        key: <User as Entity>::Column,
        val: V,
    ) -> Result<Option<User>, Self::Error>
    where
        V: sqlx::Type<Self::Database> + sqlx::Encode<'a, Self::Database> + Send + 'a,
    {
        log::debug!("Using `BasicRepostioryExt::fetch_one` implementation");

        let mut builder = sqlx::QueryBuilder::new("SELECT * FROM ");
        builder
            .push(User::table_name())
            .push(" WHERE ")
            .push(key.as_ref())
            .push(" = ")
            .push_bind(val)
            .push(" LIMIT 1");

        let query = builder.build_query_as();

        log::debug!("Query built: {}. Executing", query.sql());
        let mut rows = query.fetch(&self.conn.pool);
        rows.try_next().await
    }
}

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

#[tokio::main]
async fn main() {
    env_logger::init();

    let conn = PgConnection::new("postgres://postgres@localhost/test")
        .await
        .unwrap();

    let repo = UserRepo::new(conn.clone());

    let me = repo
        .fetch_one(UserColumn::Email, "andrey@aidev.ru")
        .await
        .unwrap()
        .unwrap();

    println!("{:?}", me);
}
