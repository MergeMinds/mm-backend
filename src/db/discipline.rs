use uuid::Uuid;

use crate::models;

use super::core::PgConnection;

impl PgConnection {
    pub async fn add_discipline(
        &self,
        name: &str,
    ) -> sqlx::Result<models::Discipline> {
        log::trace!("Inserting new discipline");

        let id = Uuid::new_v4();

        sqlx::query_as!(
            models::Discipline,
            "INSERT INTO discipline (id, name) VALUES ($1, $2)",
            id,
            name,
        )
        .execute(&self.pool)
        .await?;

        log::trace!("Inserted new discipline!");

        Ok(models::Discipline {
            id,
            name: name.to_string(),
        })
    }

    pub async fn get_discipline_by_id(
        &self,
        id: Uuid,
    ) -> sqlx::Result<models::Discipline> {
        log::trace!("Searching for discipline by given id");

        let result = sqlx::query_as!(
            models::Discipline,
            "SELECT * FROM discipline WHERE id = $1",
            id,
        )
        .fetch_one(&self.pool)
        .await;

        log::trace!("Discipline found!");

        result
    }

    pub async fn get_disciplines(
        &self,
    ) -> sqlx::Result<Vec<models::Discipline>> {
        log::trace!("Discipline getting");

        let result =
            sqlx::query_as!(models::Discipline, "SELECT * FROM discipline")
                .fetch_all(&self.pool)
                .await;

        log::trace!("Discipline received");

        result
    }

    pub async fn update_discipline_name(
        &self,
        id: Uuid,
        name: &str,
    ) -> sqlx::Result<()> {
        log::trace!("Discipline name updating");

        sqlx::query_as!(
            models::Discipline,
            "UPDATE discipline SET name = $2 WHERE id = $1",
            id,
            name,
        )
        .execute(&self.pool)
        .await?;

        log::trace!("Discipline updated!");

        Ok(())
    }

    pub async fn delete_discipline_by_id(&self, id: Uuid) -> sqlx::Result<()> {
        log::trace!("Deleting discipline");

        sqlx::query_as!(
            models::Discipline,
            "DELETE FROM discipline WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;

        log::trace!("Discipline deleted!");

        Ok(())
    }
}
