// use actix_web::dev::ServiceRequest;
// use actix_web::Error;
// use actix_web::http::header;
// use actix_web_httpauth::extractors::bearer::BearerAuth;
//
//
// use jsonwebtoken::{encode, Header, EncodingKey, Algorithm, Validation, decode, DecodingKey};
//
// use crate::config::CONFIG;
// use crate::model::Claims;
// // use crate::model::user::User;
// use crate::util::error::CustomError;

use std::time::Duration;
use argon2::{
    Argon2,
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
};
use fastdate::{DateTime, DurationFrom};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use log::debug;
use crate::CONFIG;
use crate::config::ED25519_KEY;
use crate::model::Claims;
use crate::model::user::User;
use crate::util::error::CustomError;

/// 获取 PHC 字符串
pub fn get_phc_string(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();
    // Hash password to PHC string ($argon2id$v=19$...)
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

/// 校验密码和哈希
pub fn verify_password(password: &str, password_hash: &str) -> bool {
    // NOTE: hash params from `parsed_hash` are used instead of what is configured in the `Argon2` instance.
    let parsed_hash = PasswordHash::new(password_hash).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}


// // 身份验证具体处理方法
// pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
//     let token = credentials.token();
//     let mut validation = Validation::new(Algorithm::HS384);
//     validation.set_issuer(&[&CONFIG.token_issuer]);
//     let result = decode::<Claims>(token, &DecodingKey::from_secret(CONFIG.token_secret.as_ref()), &validation)
//         .map_err(|e| {
//             let host = &*req.headers().get(header::HOST).unwrap().to_str().unwrap();
//             CustomError::UnauthorizedError {
//                 realm: host,
//                 error: "Unauthorized",
//                 message: &*e.to_string(),
//             }
//         })?;
//     debug!("{:?}", result.claims);
//
//     Ok(req)
// }

// 签发 token
pub fn sign_token(user: &User) -> Result<String, actix_web::Error> {
    let next_week = DateTime::utc() + Duration::from_day(7);
    debug!("过期时间 ==> {:?}", next_week);

    let claims = Claims {
        exp: next_week.unix_timestamp() as usize,
        iss: CONFIG.TOKEN_ISSUER.to_owned(),
        id: user.uid,
        email: user.email.to_owned(),
    };
    let header = Header::new(Algorithm::EdDSA);
    let token = jsonwebtoken::encode(&header, &claims, &ED25519_KEY)
        .map_err(|e| CustomError::InternalError { message: e.to_string()})?;

    Ok(token)
}
