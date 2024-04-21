use crate::{db::core::PgConnection, models};

use uuid::Uuid;

impl PgConnection {
    pub async fn add_user(
        &self,
        user: models::SignUpCredentials,
    ) -> sqlx::Result<()> {
        log::trace!("Inserting new user");

        let mut tx = self.pool.begin().await?;

        sqlx::query_as!(
            models::SignUpCredentials,
            "INSERT INTO users (id, email, name, surname, patronymic, role, password)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            Uuid::new_v4(),
            user.email,
            user.name,
            user.surname,
            user.patronymic,
            user.role as models::UserRole,
            &user.password.as_bytes(),
        ).execute(&mut *tx)
        .await?;

        tx.commit().await?;

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
            "SELECT id, email, name, surname, patronymic, role as \"role: _\", password FROM users
             WHERE email = $1",
            creds.email,
        ).fetch_optional(&self.pool)
        .await?
        .ok_or(sqlx::Error::RowNotFound)
    }
}
