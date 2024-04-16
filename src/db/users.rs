use crate::{
    db::core::PgConnection,
    models,
    utils::core::{Error, Result},
};

use uuid::Uuid;

impl PgConnection {
    pub async fn add_user(
        &self,
        user: models::SignUpCredentials,
    ) -> Result<()> {
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

    pub async fn verify_creds(
        &self,
        creds: models::SignInCredentials,
    ) -> Result<models::User> {
        log::trace!("Searching for user by given credentials");

        let user = sqlx::query_as!(
            models::User,
            "SELECT id, email, name, surname, patronymic, role as \"role: _\", password FROM users
             WHERE email = $1",
            creds.email,
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(Error::AuthError)?;

        let utf8_hash = std::str::from_utf8(&user.password)
            .map_err(|_| Error::AuthError)?;

        if bcrypt::verify(&creds.password, utf8_hash)? {
            Ok(user)
        } else {
            Err(Error::AuthError)
        }
    }
}