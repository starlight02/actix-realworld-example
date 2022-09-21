use actix_web::{
    dev::{self, ServiceRequest},
    FromRequest, HttpRequest,
    http::header,
};
use futures_util::future;
use fastdate::{DateTime};
use crate::model::{Claims, RealWorldToken};
use crate::util::{self, error::CustomError::UnauthorizedError};

pub async fn validator(req: ServiceRequest, credentials: RealWorldToken) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let token = credentials.token;
    debug!("即将要校验的 Token => {}", &token);
    let origin = util::get_header_value_str(req.request(), header::REFERER, "");
    let result = util::validate_token(&token, origin);
    let now = DateTime::now().unix_timestamp() as usize;
    match result {
        Ok(claims) if now < claims.exp => {
            Ok(req)
        }
        Ok(_) => {
            debug!("Token 已过期！");
            let error = UnauthorizedError {
                realm: origin.to_owned(),
                error: "Unauthorized".to_owned(),
                message: "Token expired".to_owned(),
            };
            Err((error.into(), req))
        }
        Err(err) => {
            Err((err, req))
        }
    }
}

// 为 Token 实现 actix-web 提取器
impl FromRequest for RealWorldToken {
    type Error = actix_web::Error;
    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        debug!("调用提取器");
        let authorization = request.headers().get(header::AUTHORIZATION);
        let origin = util::get_header_value_str(request, header::REFERER, "");
        // 不存在 Authorization 头部则直接返回错误
        if authorization.is_none() {
            return future::err(UnauthorizedError {
                realm: origin.to_owned(),
                error: "Unauthorized".to_owned(),
                message: "Authentication header is required!".to_owned(),
            }.into());
        }
        let header = authorization.unwrap();
        // "Token *" length
        if header.len() < 7 {
            return future::err(UnauthorizedError {
                realm: origin.to_owned(),
                error: "Unauthorized".to_owned(),
                message: "Invalid header value".to_owned(),
            }.into());
        }
        let mut parts = header.to_str().unwrap().splitn(2, ' ');
        // 匹配token
        match parts.next() {
            Some(scheme) if scheme == "Token" => {}
            _ => return future::err(UnauthorizedError {
                realm: origin.to_owned(),
                error: "Unauthorized".to_owned(),
                message: "Missing authorization scheme".to_owned(),
            }.into()),
        }
        let token = parts.next().ok_or(UnauthorizedError {
            realm: origin.to_owned(),
            error: "Unauthorized".to_owned(),
            message: "Invalid header value".to_owned(),
        }).unwrap();

        future::ok(RealWorldToken { token: token.to_owned() })
    }
}

// 为 Claims 实现 actix-web 提取器
impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let authorization = request.headers().get(header::AUTHORIZATION).unwrap().to_str().unwrap();
        let list: Vec<&str> = authorization.splitn(2, ' ').collect();
        let origin = util::get_header_value_str(request, header::REFERER, "");
        let result = util::validate_token(list[1], origin);
        if let Err(e) = result {
            return future::err(e);
        }

        future::ok(result.unwrap())
    }
}

