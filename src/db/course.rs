use super::core::PgConnection;
use crate::models;
use uuid::Uuid;

impl PgConnection {
    pub async fn add_course(
        &self,
        course: models::CourseIn,
    ) -> sqlx::Result<models::Course> {
        log::trace!("Inserting new course");

        let id = Uuid::new_v4();
        let created_at = chrono::Utc::now().naive_utc();

        sqlx::query_as!(
            models::Course,
            "INSERT INTO course (id, discipline_id, owner_id, name, created_at) VALUES ($1, $2, $3, $4, $5)",
            id,
            course.discipline_id,
            course.owner_id,
            course.name,
            created_at,
        )
        .execute(&self.pool)
        .await?;

        log::trace!("Inserted new course!");

        Ok(models::Course {
            id,
            created_at,
            discipline_id: course.discipline_id,
            owner_id: course.owner_id,
            name: course.name,
        })
    }

    pub async fn get_course_by_id(
        &self,
        id: Uuid,
    ) -> sqlx::Result<models::Course> {
        log::trace!("Searching for course by given id");

        let result = sqlx::query_as!(
            models::Course,
            "SELECT * FROM course WHERE id = $1",
            id,
        )
        .fetch_one(&self.pool)
        .await;

        log::trace!("Course found by id!");

        result
    }

    pub async fn get_courses(&self) -> sqlx::Result<Vec<models::Course>> {
        log::trace!("Course getting");

        let result = sqlx::query_as!(models::Course, "SELECT * FROM course")
            .fetch_all(&self.pool)
            .await;

        log::trace!("Course received!");

        result
    }

    pub async fn update_course_by_id(
        &self,
        id: Uuid,
        course: models::CourseIn,
    ) -> sqlx::Result<()> {
        log::trace!("Course updating");

        sqlx::query_as!(
            models::Course,
            "UPDATE course SET discipline_id = $1, owner_id = $2, name = $3 WHERE id = $4",
            course.discipline_id,
            course.owner_id,
            course.name,
            id,
        )
        .execute(&self.pool)
        .await?;

        log::trace!("Course updated!");

        Ok(())
    }

    pub async fn delete_course_by_id(&self, id: Uuid) -> sqlx::Result<()> {
        log::trace!("Deleting course");

        sqlx::query_as!(models::Course, "DELETE FROM course WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        log::trace!("Course deleted!");

        Ok(())
    }
}
