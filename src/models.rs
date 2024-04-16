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
    pub name: String,
    pub surname: String,
    pub patronymic: Option<String>,
    pub password: Vec<u8>,
    pub role: UserRole,
}

#[derive(utoipa::ToSchema, Serialize, Deserialize)]
pub struct SignUpCredentials {
    pub email: String,
    pub name: String,
    pub surname: String,
    pub patronymic: Option<String>,
    pub role: UserRole,
    pub password: String,
}

#[derive(utoipa::ToSchema, Serialize, Deserialize)]
pub struct SignInCredentials {
    pub email: String,
    pub password: String,
}
