use actix_http::body::BoxBody;
use actix_web::{
    dev,
    http::{StatusCode, header},
    middleware::ErrorHandlerResponse,
};

// 重新格式化 actix-web 的错误消息
pub fn format_response<B>(mut response: dev::ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    // 重写请求头的 content-type
    response.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json; charset=utf-8"),
    );
    // 重写 Http StatusCode 为 422
    response.response_mut().head_mut().status = StatusCode::UNPROCESSABLE_ENTITY;
    // 获取框架的错误信息
    let error_message: String = match response.response().error() {
        Some(e) => e.to_string(),
        None => String::from("Unknown Error")
    };
    // 格式化响应体为要求的返回格式
    let body = json!({
        "error": {
            "body": [error_message]
        }
    }).to_string();
    let new_response = response.map_body(move |_head, _body| BoxBody::new(body));

    Ok(ErrorHandlerResponse::Response(new_response.map_into_right_body()))
}
