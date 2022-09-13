use std::fmt::Debug;
use actix_web::{
    http::{header::ContentType, StatusCode},
    error, HttpResponse,
};
use crate::model::{ResponseError, ResponseMessage};

#[derive(Debug, derive_more::Display, derive_more::Error)]
pub enum CustomError {
    #[display(fmt = "validation error: {}", message)]
    ValidationError { message: String },

    #[display(fmt = "{}: {}", error, message)]
    UnauthorizedError {
        realm: String,
        error: String,
        message: String,
    },

    #[display(fmt = "internal error: {}", message)]
    InternalError { message: String },
}

// 为自定义错误实现 ResponseError 以可返回 HTTP 错误
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

        if let CustomError::UnauthorizedError { realm, error, message } = self {
            builder.insert_header((
                "WWW-Authenticate",
                format!("Bearer realm=\"{}\", error=\"{}\", error_description=\"{}\"", realm, error, message)
            ));
        }

        builder
            .insert_header(ContentType::json())
            .json(ResponseMessage {
                errors: ResponseError {
                    body: vec![
                        self.to_string()
                    ]
                }
            })
    }
}
