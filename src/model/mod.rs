use actix_http::body::BoxBody;
use actix_web::{
    dev, Error, HttpRequest, HttpResponse, Responder, FromRequest,
    http::header,
    error::{ErrorUnauthorized},
};
use futures_util::future;
use crate::util::error::CustomError::UnauthorizedError;

pub mod user;

pub use user::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
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

// 请求的载荷
#[derive(Debug, serde::Deserialize)]
pub struct RequestPayload {
    pub user: NewUser,
}

#[derive(Debug, serde::Deserialize)]
pub struct RequestCredentials {
    pub user: LoginCredentials,
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
            .content_type(header::ContentType::json())
            .body(body)
    }
}

// 为 token 实现 actix-web 提取器
impl FromRequest for RealWorldToken {
    type Error = Error;
    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let option = request.headers().get(header::AUTHORIZATION);
        let host = request.headers().get(header::HOST).unwrap().to_str().unwrap();
        // 不存在头部则直接返回错误
        if option.is_none() {
            return future::err(ErrorUnauthorized(UnauthorizedError {
                realm: host.to_owned(),
                error: "Unauthorized".to_owned(),
                message: "Authentication header is required!".to_owned(),
            }));
        }
        let header = option.unwrap();
        // "Token *" length
        if header.len() < 7 {
            return future::err(ErrorUnauthorized(UnauthorizedError {
                realm: host.to_owned(),
                error: "Unauthorized".to_owned(),
                message: "Invalid header value".to_owned(),
            }));
        }
        let mut parts = header.to_str().unwrap().splitn(2, ' ');
        // 匹配token
        match parts.next() {
            Some(scheme) if scheme == "Token" => {}
            _ => return future::err(ErrorUnauthorized(UnauthorizedError {
                realm: host.to_owned(),
                error: "Unauthorized".to_owned(),
                message: "Missing authorization scheme".to_owned(),
            })),
        }
        let token = parts.next().ok_or(UnauthorizedError {
            realm: host.to_owned(),
            error: "Unauthorized".to_owned(),
            message: "Invalid header value".to_owned(),
        }).unwrap().to_owned();

        future::ok(RealWorldToken { token })
    }
}
