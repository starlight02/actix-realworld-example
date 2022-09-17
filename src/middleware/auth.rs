use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web::http::header;
use log::debug;
use crate::model::{RealWorldToken};
use crate::util;

pub async fn validator(req: ServiceRequest, credentials: RealWorldToken) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token;
    debug!("即将要校验的 Token => {}", &token);
    let host = req.headers().get(header::HOST).unwrap().to_str().unwrap();
    let result = util::validate_token(&token, host);

    match result {
        Ok(_claims) => {
            // TODO
            Ok(req)
        }
        Err(err) => {
            Err((err, req))
        }
    }
}
