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
