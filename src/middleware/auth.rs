use actix_web::{
    dev::{self, ServiceRequest},
    FromRequest, HttpRequest,
    http::header,
    error::ErrorBadRequest,
};
use fastdate::{DateTime};
use futures_util::future::{self, LocalBoxFuture};
use crate::model::{Claim, RealWorldToken};
use crate::util::{self, error::CustomError::UnauthorizedError};

pub async fn validator(req: ServiceRequest, credentials: actix_web::Result<RealWorldToken>) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let origin = util::get_header_value_str(req.request(), header::REFERER, "");
    let token = match credentials {
        Err(e) => {
            debug!("捕获到错误 ==> {:#?}", e);
            return Err((UnauthorizedError {
                realm: origin.to_owned(),
                message: e.to_string(),
            }.into(), req));
        },
        Ok(token) => token,
    };
    debug!("即将要校验的 Token => {:#?}", &token);
    let RealWorldToken { token, scheme } = token;
    match scheme.as_str() {
        "Token" => {},
        _ => return Err((UnauthorizedError {
            realm: origin.to_owned(),
            message: "Invalid header value".to_owned(),
        }.into(), req)),
    };

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
    type Future = future::Ready<actix_web::Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        debug!("调用 Token 提取器");
        let authorization = request.headers().get(header::AUTHORIZATION);
        if authorization.is_none() {
            return future::err(ErrorBadRequest("Authentication required!"))
        }
        let mut parts = authorization.unwrap().to_str().unwrap().splitn(2, ' ');

        let scheme = parts.next().map(|s| s.to_owned());
        if scheme.is_none() || scheme.as_ref().is_some_and(|s| s.is_empty()) {
            return future::err(ErrorBadRequest("Missing authorization scheme"));
        }

        let token = parts.next().map(|s| s.to_owned());
        if token.is_none() || token.as_ref().is_some_and(|s| s.is_empty()) {
            return future::err(ErrorBadRequest("Invalid header value"));
        }

        let scheme = scheme.unwrap();
        let token = token.unwrap();
        future::ok(RealWorldToken { scheme, token })
    }
}

// 为 Claims 实现 actix-web 提取器
impl FromRequest for Claim {
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, actix_web::Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        debug!("调用 Token 载荷提取器");
        let request = request.to_owned();
        Box::pin(async move {
            let RealWorldToken { token, .. } = RealWorldToken::extract(&request).await?;
            let origin = util::get_header_value_str(&request, header::REFERER, "");

            util::validate_token(&token, origin)
        })
    }
}
