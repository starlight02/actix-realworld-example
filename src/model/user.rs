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
    pub image: Option<String>,
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
    pub image: Option<String>,
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

#[derive(Debug, serde::Serialize)]
pub struct ResponseData {
    pub body: String,
}

impl ResponseData {
    pub fn new<T>(property_name: &str, object: T) -> Self
        where T: serde::Serialize
    {
        // 如果要获取对象的类型的字符串，可以尝试下面的方法
        // let name = std::any::type_name::<T>();
        Self {
            body: json!({property_name: object}).to_string()
        }
    }
}

// 为返回体实现 actix Responder
impl Responder for ResponseData {
    type Body = actix_http::body::BoxBody;

    fn respond_to(self, _request: &HttpRequest) -> HttpResponse<Self::Body> {
        // Create HttpResponse and set Content Type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(self.body)
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
