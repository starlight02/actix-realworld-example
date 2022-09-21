use actix_web::{HttpRequest, HttpResponse, Responder, http::header::ContentType};
use rbatis::rbdc::datetime::FastDateTime;

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
    pub image: Option<String>,
    pub deleted: Option<bool>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct LoginPayload {
    pub user: LoginCredentials,
}

#[derive(Debug, serde::Deserialize)]
pub struct SignUpPayload {
    pub user: NewUser,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateUserPayload {
    pub user: UpdateUser,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Profile {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResponseUser {
    pub user: Option<User>,
}

// 为返回体实现 actix Responder
impl Responder for ResponseUser {
    type Body = actix_http::body::BoxBody;

    fn respond_to(self, _request: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create HttpResponse and set Content Type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResponseProfile {
    pub profile: Option<Profile>,
}

impl Responder for ResponseProfile {
    type Body = actix_http::body::BoxBody;

    fn respond_to(self, _request: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserFollow {
    pub uid: u32,
    pub follow_uid: u32,
}

impl_update!(UpdateUser {}, r#""user""#);
impl_select!(UserTable {}, r#""user""#);
impl_select!(UserFollow {select_follow(uid:u32, follow_uid:u32) -> Option => "`WHERE uid = #{uid} AND follow_uid = #{follow_uid}`"});