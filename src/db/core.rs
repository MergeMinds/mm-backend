use crate::db::entity::Entity;
use futures::TryStreamExt;
use serde::Serialize;
use sqlx::{database::HasArguments, Execute, Executor, IntoArguments};
use std::fmt::Display;

pub trait Repository<D, E>
where
    D: sqlx::Database + Send,
    for<'r> <D as HasArguments<'r>>::Arguments: IntoArguments<'r, D>,
    for<'r> E: sqlx::FromRow<'r, <D as sqlx::Database>::Row>
        + Entity
        + Send
        + Unpin
        + Serialize,
    for<'c> &'c mut <D as sqlx::Database>::Connection:
        Executor<'c, Database = D>,
{
    fn get_conn_pool(&self) -> &sqlx::Pool<D>;

    async fn fetch_one<K, V>(&self, key: K, value: V) -> sqlx::Result<Option<E>>
    where
        K: Send
            + for<'q> sqlx::Encode<'q, D>
            + sqlx::Type<D>
            + Display
            + 'static,
        V: Send + for<'q> sqlx::Encode<'q, D> + sqlx::Type<D> + 'static,
    {
        log::debug!("Using `BasicRepostioryExt::fetch_one` implementation");
        let pool = self.get_conn_pool();

        let mut builder = sqlx::QueryBuilder::new("SELECT * FROM ")
            .push(E::table_name())
            .push(" WHERE ")
            .push_bind(key)
            .push(" = ")
            .push_bind(value)
            .push(" LIMIT 1");

        let query = builder.build_query_as();

        log::debug!("Query built: {}. Executing", query.sql());
        let mut rows = query.fetch(pool);
        rows.try_next().await
    }

    // async fn fetch_many<'q>(&self) -> sqlx::Result<Vec<E>> {
    //     log::debug!("Using `BasicRepostioryExt::fetch_many` implementation");
    //     let pool = self.get_conn_pool();

    //     let mut builder = sqlx::QueryBuilder::new("SELECT * FROM ");
    //     builder.push(E::table_name());

    //     let query = builder.build_query_as::<E>();
    //     log::debug!("Query built: {}. Executing", query.sql());

    //     let rows = query.fetch(&pool);
    //     rows.try_collect().await
    // }

    // async fn fetch_many_with_cond<'q, K, V>(
    //     &self,
    //     key: K,
    //     value: V,
    // ) -> sqlx::Result<Vec<E>>
    // where
    //     K: Display,
    //     V: 'q + Send + sqlx::Encode<'q, MySql> + sqlx::Type<MySql>,
    // {
    //     log::debug!(
    //         "Using `BasicRepostioryExt::fetch_many_with_cond` implementation"
    //     );
    //     let pool = self.get_conn_pool();

    //     let mut builder = sqlx::QueryBuilder::new("SELECT * FROM ");
    //     builder.push(E::table_name());

    //     builder.push(" WHERE ");
    //     builder.push(key);
    //     builder.push(" = ");
    //     builder.push_bind(value);

    //     let query = builder.build_query_as::<E>();
    //     log::debug!("Query built: {}. Executing", query.sql());

    //     let rows = query.fetch(&pool);
    //     rows.try_collect().await
    // }
    // async fn fetch_many_paginated<'q>(
    //     &self,
    //     page: i64,
    //     size: i64,
    // ) -> sqlx::Result<Vec<E>> {
    //     log::debug!(
    //         "Using `BasicRepostioryExt::fetch_many_paginated` implementation"
    //     );
    //     let pool = self.get_conn_pool();

    //     let mut builder = sqlx::QueryBuilder::new("SELECT * FROM ");
    //     builder.push(E::table_name());

    //     builder.push(" OFFSET ");
    //     builder.push_bind(page * size);
    //     builder.push(" LIMIT ");
    //     builder.push_bind(size);

    //     let query = builder.build_query_as::<E>();
    //     log::debug!("Query built: {}. Executing", query.sql());

    //     let rows = query.fetch(&pool);
    //     rows.try_collect().await
    // }
    // async fn fetch_unsafe<'a>(&self, cond: &'a str) -> sqlx::Result<Vec<E>> {
    //     log::debug!("Using `BasicRepostioryExt::fetch_unsafe` implementation");
    //     let pool = self.get_conn_pool();

    //     let mut builder = sqlx::QueryBuilder::new("SELECT * FROM ");
    //     builder.push(E::table_name());
    //     builder.push(" WHERE ");
    //     builder.push(cond);

    //     let query = builder.build_query_as::<E>();
    //     log::debug!("Query built: {}. Executing", query.sql());

    //     let rows = query.fetch(&pool);
    //     rows.try_collect().await
    // }

    // async fn add(
    //     &self,
    //     entities: impl IntoIterator<Item = E>,
    // ) -> sqlx::Result<()> {
    //     log::debug!("Using `BasicRepostioryExt::add` implementation");
    //     for e in entities {
    //         if let serde_json::Value::Object(obj) =
    //             serde_json::to_value(e).unwrap()
    //         {
    //             let mut builder =
    //                 sqlx::QueryBuilder::<MySql>::new("INSERT INTO ");

    //             builder.push(E::table_name());
    //             builder.push(" (");

    //             let mut keys = obj.iter().map(|(k, _)| k);
    //             if let Some(key) = keys.next() {
    //                 builder.push(key);
    //             }
    //             for key in keys {
    //                 builder.push(", ");
    //                 builder.push(key);
    //             }
    //             builder.push(") ");

    //             let values = obj.into_iter().map(|(_, v)| v);
    //             builder.push_values(
    //                 vec![values].into_iter(),
    //                 |mut b, values| {
    //                     for value in values {
    //                         b.push_bind(value);
    //                     }
    //                 },
    //             );

    //             let query = builder.build();
    //             log::debug!("Query built: {}. Executing", query.sql());
    //             query.execute(&self.get_conn_pool()).await?;
    //         } else {
    //             log::warn!(
    //                 "An attempt to save something other than object to the database. Ignoring"
    //             );
    //         }
    //     }
    //     Ok(())
    // }
    // async fn update_with_cond<'q, K, V>(
    //     &self,
    //     update: serde_json::Value,
    //     key: K,
    //     value: V,
    // ) -> sqlx::Result<()>
    // where
    //     K: Display,
    //     V: 'q + Send + sqlx::Encode<'q, MySql> + sqlx::Type<MySql>,
    // {
    //     log::debug!("Using `BasicRepostioryExt::update` implementation");
    //     let mut builder = sqlx::QueryBuilder::<MySql>::new("UPDATE ");
    //     let pool = self.get_conn_pool();
    //     builder.push(E::table_name());

    //     if let serde_json::Value::Object(obj) = update {
    //         let mut update_it = obj.into_iter();
    //         if let Some((k, v)) = update_it.next() {
    //             builder.push(" SET ");
    //             builder.push(k);
    //             builder.push(" = ");
    //             builder.push_bind(v);
    //         }
    //         for (k, v) in update_it {
    //             builder.push(", ");
    //             builder.push(k);
    //             builder.push(" = ");
    //             builder.push_bind(v);
    //         }

    //         builder.push(" WHERE ");
    //         builder.push(key);
    //         builder.push(" = ");
    //         builder.push_bind(value);

    //         let query = builder.build();
    //         log::debug!("Query built: {}. Executing", query.sql());
    //         query.execute(&pool).await?;
    //     } else {
    //         log::warn!(
    //                 "An attempt to update table rows with something other than object. Ignoring"
    //             );
    //     }
    //     Ok(())
    // }
    // async fn update_unsafe<'a, 'q, K, V>(
    //     &self,
    //     update: serde_json::Value,
    //     cond: &'a str,
    // ) -> sqlx::Result<()>
    // where
    //     K: Display,
    //     V: 'q + Send + sqlx::Encode<'q, MySql> + sqlx::Type<MySql>,
    // {
    //     log::debug!("Using `BasicRepostioryExt::update_unsafe` implementation");
    //     let mut builder = sqlx::QueryBuilder::<MySql>::new("UPDATE ");
    //     let pool = self.get_conn_pool();
    //     builder.push(E::table_name());

    //     if let serde_json::Value::Object(obj) = update {
    //         let mut update_it = obj.into_iter();
    //         if let Some((k, v)) = update_it.next() {
    //             builder.push(" SET ");
    //             builder.push(k);
    //             builder.push(" = ");
    //             builder.push_bind(v);
    //         }
    //         for (k, v) in update_it {
    //             builder.push(", ");
    //             builder.push(k);
    //             builder.push(" = ");
    //             builder.push_bind(v);
    //         }

    //         builder.push(" WHERE ");
    //         builder.push(cond);

    //         let query = builder.build();
    //         log::debug!("Query built: {}. Executing", query.sql());
    //         query.execute(&pool).await?;
    //     } else {
    //         log::warn!(
    //                 "An attempt to update table rows with something other than object. Ignoring"
    //             );
    //     }

    //     Ok(())
    // }
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
