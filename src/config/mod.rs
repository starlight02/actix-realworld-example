use once_cell::sync::Lazy;
use application::ApplicationConfig;

pub mod router;
pub mod log;
pub mod database;
pub mod application;

pub static CONFIG: Lazy<ApplicationConfig> = Lazy::new(ApplicationConfig::default);
