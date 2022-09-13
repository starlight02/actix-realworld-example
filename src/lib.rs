#[macro_use]
extern crate rbatis;

pub mod config;
pub mod controller;
pub mod service;
pub mod model;
pub mod middleware;
pub mod util;

pub use config::database;
pub use config::log as app_log;
pub use config::router;
pub use config::CONFIG;
