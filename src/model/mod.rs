use actix_http::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use crate::model::user::{NewUser, User};

pub mod user;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    // 必要，过期时间，UTC 时间戳
    pub exp: usize,
    // 可选，签发人
    pub iss: &'static str,
    pub id: u32,
    pub uname: &'static str,
}

// 请求的载荷
#[derive(Debug, serde::Deserialize)]
pub struct RequestPayload {
    pub user: NewUser,
}

// 统一的响应
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResponseData {
    pub user: Option<User>,
    // pub article: User,
    // pub comment: User,
    // pub comments: User,
    // pub articles: User,
}

// 统一的错误响应
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponseMessage {
    pub errors: ResponseError,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponseError {
    pub body: Vec<String>,
}

// 为返回体实现 actix Responder
impl Responder for ResponseData {
    type Body = BoxBody;

    fn respond_to(self, _request: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create HttpResponse and set Content Type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
