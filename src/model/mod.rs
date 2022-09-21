use actix_web::{
    HttpRequest, HttpResponse, Responder,
    http::header,
};

pub mod user;

pub use user::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    // 必要，过期时间，UTC 时间戳
    pub exp: usize,
    // 可选，签发人
    pub iss: String,
    pub id: u32,
    pub email: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RealWorldToken {
    pub token: String,
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
