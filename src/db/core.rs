use crate::db::entity::Entity;
use futures::TryStreamExt;
use serde::Serialize;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::query::QueryAs;
use sqlx::types::JsonValue;
use sqlx::Execute;
use sqlx::Postgres;

pub trait PgRepository<E>
where
    E: for<'q> sqlx::FromRow<'q, PgRow> + Entity,
{
    fn get_conn_pool(&self) -> &PgPool;

    async fn fetch_one<'q, K, V>(
        &self,
        key: K,
        value: V,
    ) -> sqlx::Result<Option<E>>
    where
        K: Send + sqlx::Encode<'q, Postgres> + sqlx::Type<Postgres> + 'q,
        V: Send + sqlx::Encode<'q, Postgres> + sqlx::Type<Postgres> + 'q,
        QueryAs<'q, Postgres, E, Postgres>: Clone,
        E: Unpin + Send,
    {
        log::debug!("Using `BasicRepostioryExt::fetch_one` implementation");

        let builder = sqlx::query_as("SELECT * FROM $1 WHERE $2 = $3 LIMIT 1");

        let query = builder.bind(E::table_name()).bind(key).bind(value);

        let pool = self.get_conn_pool();
        let mut rows = query.fetch(pool);
        rows.try_next().await
    }

    // TODO(nrydanov): WHERE should be able to take some EXPR instead of just
    // pair {key, value}, see https://github.com/MergeMinds/mm-backend/issues/24
    async fn fetch_many<'q, K, V>(
        &self,
        key: K,
        value: V,
    ) -> sqlx::Result<Vec<E>>
    where
        K: Send + sqlx::Encode<'q, Postgres> + sqlx::Type<Postgres> + 'q,
        V: Send + sqlx::Encode<'q, Postgres> + sqlx::Type<Postgres> + 'q,
        QueryAs<'q, Postgres, E, Postgres>: Clone,
        E: Unpin + Send,
    {
        log::debug!("Using `BasicRepostioryExt::fetch_many` implementation");

        let builder = sqlx::query_as("SELECT * FROM $1 WHERE $2 = $3");

        let query = builder.bind(E::table_name()).bind(key).bind(value);

        let pool = self.get_conn_pool();
        let rows = query.fetch(pool);
        rows.try_collect().await
    }

    async fn add<'a>(
        &self,
        entities: impl IntoIterator<Item = E>,
    ) -> sqlx::Result<()>
    where
        E: Serialize,
        sqlx::types::Json<&'a JsonValue>: sqlx::Encode<'a, Postgres>,
    {
        log::debug!("Using `BasicRepostioryExt::add` implementation");
        for e in entities {
            if let serde_json::Value::Object(obj) =
                serde_json::to_value(e).unwrap()
            {
                let mut builder = sqlx::QueryBuilder::new("INSERT INTO ");

                builder.push_bind(E::table_name());
                builder.push(" (");

                let mut keys = vec![];
                let mut values = vec![];

                obj.iter().for_each(|(k, v)| {
                    keys.push(k);
                    values.push(v);
                });

                builder.push_bind(keys[0]);

                keys[1..].iter().for_each(|k| {
                    builder.push(", ");
                    builder.push_bind(k);
                });

                builder.push(") ");

                builder.push_values(
                    vec![values].into_iter(),
                    |mut b, values| {
                        for value in values {
                            b.push_bind(value);
                        }
                    },
                );

                let query = builder.build();
                log::debug!("Generated query: {:?}", query.sql());
                let _ = query.execute(self.get_conn_pool()).await;
                log::debug!("Query executed successfully")
            } else {
                // NOTE(nrydanov: I can't believe this is actually possible
                panic!("An attempt to save something other than object to the database o_O");
            }
        }
        Ok(())
    }
}

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
