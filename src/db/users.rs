use crate::{db::core::PgConnection, models};

use uuid::Uuid;

impl PgConnection {
    pub async fn add_user(
        &self,
        user: models::SignUpCredentials,
    ) -> sqlx::Result<()> {
        log::trace!("Inserting new user");

        sqlx::query_as!(
            models::SignUpCredentials,
            "INSERT INTO users (id, username, email, password, name, surname, 
             date_of_birth, created_at, last_online)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            Uuid::new_v4(),
            user.username,
            user.email,
            &user.password.as_bytes(),
            user.name,
            user.surname,
            user.date_of_birth,
            chrono::Utc::now().naive_utc(),
            chrono::Utc::now().naive_utc(),
        )
        .execute(&self.pool)
        .await?;

        log::trace!("Inserted new user");
        Ok(())
    }

    pub async fn get_user_by_creds(
        &self,
        creds: &models::SignInCredentials,
    ) -> sqlx::Result<models::User> {
        log::trace!("Searching for user by given credentials");

        sqlx::query_as!(
            models::User,
            "SELECT * FROM users WHERE username = $1 OR email = $1",
            creds.login,
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(sqlx::Error::RowNotFound)
    }
}
