#![feature(is_some_with)]
#![feature(option_result_contains)]
#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

pub mod config;
pub mod controller;
pub mod service;
pub mod model;
pub mod middleware;
pub mod util;

pub use middleware as app_middleware;
pub use config::database;
pub use config::log as app_log;
pub use config::router;
pub use config::CONFIG;
