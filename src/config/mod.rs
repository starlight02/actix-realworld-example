use jsonwebtoken::EncodingKey;
use once_cell::sync::Lazy;
use application::ApplicationConfig;

pub mod router;
pub mod log;
pub mod database;
pub mod application;

pub static CONFIG: Lazy<ApplicationConfig> = Lazy::new(ApplicationConfig::default);
pub static ED25519_KEY: Lazy<EncodingKey> = Lazy::new(|| EncodingKey::from_ed_pem(include_bytes!("../../realworld_ed25519")).expect("Not a valid private Ed key!"));
