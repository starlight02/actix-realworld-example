use actix_web::{
    dev::{self, ServiceRequest},
    FromRequest, HttpRequest,
    http::header,
    error::ErrorBadRequest,
};
use futures_util::future;
use fastdate::{DateTime};
use crate::model::{Claim, RealWorldToken};
use crate::util::{self, error::CustomError::UnauthorizedError};

pub async fn validator(req: ServiceRequest, credentials: Option<RealWorldToken>) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let origin = util::get_header_value_str(req.request(), header::REFERER, "");
    let token = match credentials {
        Some(token) if token.token.is_some() || token.token.is_some() => token,
        _ => {
            return Err((UnauthorizedError {
                realm: origin.to_owned(),
                message: "Authentication required!".to_owned(),
            }.into(), req));
        },
    };
    debug!("即将要校验的 Token => {:#?}", &token);
    let RealWorldToken { token, scheme } = token;

    match scheme.as_deref() {
        Some("Token") => {},
        Some("") => return Err((UnauthorizedError {
            realm: origin.to_owned(),
            message: "Missing authorization scheme".to_owned(),
        }.into(), req)),
        _ => return Err((UnauthorizedError {
            realm: origin.to_owned(),
            message: "Invalid header value".to_owned(),
        }.into(), req)),
    };
    if token.is_none(){
        return Err((UnauthorizedError {
            realm: origin.to_owned(),
            message: "Invalid header value".to_owned(),
        }.into(), req));
    }

    let token = token.unwrap();
    let result = util::validate_token(&token, origin);
    let now = DateTime::now().unix_timestamp() as usize;
    match result {
        Ok(claims) if now < claims.exp => {
            // TODO Doing more
            Ok(req)
        }
        Ok(_) => {
            debug!("Token 已过期！");
            let error = UnauthorizedError {
                realm: origin.to_owned(),
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
        debug!("调用 Token 提取器");
        let authorization = request.headers().get(header::AUTHORIZATION);
        if authorization.is_none() {
            return future::err(ErrorBadRequest("Authentication required!"))
        }
        let mut parts = authorization.unwrap().to_str().unwrap().splitn(2, ' ');
        let scheme = parts.next().map(|s| s.to_owned());
        let token = parts.next().map(|s| s.to_owned());

        future::ok(RealWorldToken { scheme, token })
    }
}

// 为 Claims 实现 actix-web 提取器
impl FromRequest for Claim {
    type Error = actix_web::Error;
    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        debug!("调用 Token 载荷提取器");
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

