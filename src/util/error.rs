use std::fmt::Debug;
use actix_web::{
    http::{header::ContentType, StatusCode},
    error, HttpResponse,
};

#[derive(Debug, derive_more::Display, derive_more::Error)]
pub enum CustomError {
    #[display(fmt = "Validation error: {}", message)]
    ValidationError { message: String },

    #[display(fmt = "Unauthorized: {}", message)]
    UnauthorizedError {
        realm: String,
        message: String,
    },

    #[display(fmt = "Internal error: {}", message)]
    InternalError { message: String },
}

// 为自定义错误实现 ResponseError 以可返回 HTTP 错误
// 这里 Clion 显示错误，但实际上 build 是没有问题的
impl error::ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::ValidationError { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            CustomError::UnauthorizedError { .. } => StatusCode::UNAUTHORIZED,
            CustomError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let mut builder = HttpResponse::build(self.status_code());

        if let CustomError::UnauthorizedError { realm, message, .. } = self {
            debug!("错误消息 ==> {}", message);
            let error_message = format!("Token realm=\"{}\", error=\"Unauthorized\", error_description=\"{}\"", realm, message);
            builder.insert_header(("WWW-Authenticate", error_message.as_str()));
        }

        let response_message = json!({
            "error": {
                "body": [self.to_string()]
            }
        }).to_string();

        builder
            .content_type(ContentType::json())
            .body(response_message)
    }
}
