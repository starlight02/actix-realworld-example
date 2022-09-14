use rbatis::rbdc::datetime::{FastDateTime};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct UserTable {
    pub uid: u32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub nickname: Option<String>,
    pub bio: Option<String>,
    pub images: Option<String>,
    pub created_time: FastDateTime,
    pub updated_time: FastDateTime,
    pub deleted: bool,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct User {
    pub uid: u32,
    pub email: String,
    pub username: String,
    pub nickname: Option<String>,
    pub bio: Option<String>,
    pub images: Option<String>,
    pub token: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UpdateUser {
    pub uid: u32,
    pub email: Option<String>,
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub images: Option<String>,
    pub deleted: Option<bool>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

crud!(UpdateUser {}, "user");
