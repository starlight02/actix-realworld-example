use jsonwebtoken::{EncodingKey, DecodingKey};
use once_cell::sync::Lazy;
use application::ApplicationConfig;

pub mod router;
pub mod log;
pub mod database;
pub mod application;

pub static CONFIG: Lazy<ApplicationConfig> = Lazy::new(ApplicationConfig::default);
pub static ED25519_PRIVATE_KEY: Lazy<EncodingKey> = Lazy::new(|| EncodingKey::from_ed_pem(include_bytes!("../../realworld.pem")).expect("Not a valid EdDSA private key!"));
pub static ED25519_PUBLIC_KEY: Lazy<DecodingKey> = Lazy::new(|| DecodingKey::from_ed_pem(include_bytes!("../../realworld.pub")).expect("Not a valid EdDSA public key!"));
