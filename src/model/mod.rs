use actix_web::{
    HttpRequest, HttpResponse, Responder,
    http::header::ContentType,
};

pub mod user;

pub use user::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claim {
    // 必要，过期时间，UTC 时间戳
    pub exp: usize,
    // 可选，签发人
    pub iss: String,
    pub id: u32,
    pub email: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RealWorldToken {
    pub scheme: String,
    pub token: String,
}

// 统一响应结构体
#[derive(Debug, serde::Serialize)]
pub struct ResponseData {
    pub body: String,
}

impl ResponseData {
    pub fn new<T>(property_name: &str, data: T) -> Self
        where T: serde::Serialize
    {
        Self {
            body: json!({property_name: data}).to_string()
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