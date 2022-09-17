use std::time::Duration;
use argon2::{
    Argon2,
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
};
use fastdate::{DateTime, DurationFrom};
use jsonwebtoken::{Algorithm, Header, Validation};
use log::debug;
use crate::{
    CONFIG,
    config::{ED25519_PRIVATE_KEY, ED25519_PUBLIC_KEY},
    model::{Claims},
    util::error::CustomError,
};

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

// 签发 token
pub fn sign_token(id: u32, email: String) -> Result<String, actix_web::Error> {
    let next_week = DateTime::now() + Duration::from_day(7);
    debug!("过期时间 ==> {:#?}", next_week.to_string());

    let claims = Claims {
        exp: next_week.unix_timestamp() as usize,
        iss: CONFIG.TOKEN_ISSUER.to_owned(),
        id,
        email,
    };
    let header = Header::new(Algorithm::EdDSA);
    let token = jsonwebtoken::encode(&header, &claims, &ED25519_PRIVATE_KEY)
        .map_err(|e| CustomError::InternalError { message: e.to_string() })?;

    Ok(token)
}

// 验证 Token
pub fn validate_token(token: &str, host: &str) -> Result<Claims, actix_web::Error> {
    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.set_issuer(&[CONFIG.TOKEN_ISSUER.as_str()]);
    let result = jsonwebtoken::decode::<Claims>(token, &ED25519_PUBLIC_KEY, &validation)
        .map_err(|e| CustomError::UnauthorizedError {
            realm: host.to_owned(),
            error: "Unauthorized".to_owned(),
            message: e.to_string(),
        })?;
    debug!("Token 的载荷 => {:#?}", &result.claims);

    Ok(result.claims)
}
