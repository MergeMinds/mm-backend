use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::Type, Clone, Serialize, Deserialize)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Teacher,
    Student,
}

#[derive(utoipa::ToSchema, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub name: String,
    pub surname: String,
    pub password: Vec<u8>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub created_at: chrono::NaiveDateTime,
    pub last_online: Option<chrono::NaiveDateTime>,
}

#[derive(utoipa::ToSchema, Serialize, Deserialize)]
pub struct SignUpCredentials {
    pub email: String,
    pub username: String,
    pub name: String,
    pub surname: String,
    pub password: String,
    pub date_of_birth: Option<chrono::NaiveDate>,
}

#[derive(utoipa::ToSchema, Serialize, Deserialize)]
pub struct SignInCredentials {
    pub login: String,
    pub password: String,
}
