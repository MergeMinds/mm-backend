use uuid::Uuid;

pub enum UserRole {
    Admin,
    Teacher,
    Student,
}

#[derive(utoipa::ToSchema)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub patronymic: Option<String>,
    pub role: UserRole,
    pub password: String, // FIXME(granatam): Password should be a byte array
}
