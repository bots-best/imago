#![deny(rust_2018_idioms)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod models;
pub mod web;

/// Configuration for the actix web application
#[derive(Debug)]
pub struct AppConfig {
    pub debug: bool,
    pub host: String,
    pub port: u64,
    pub workers: usize,
}

impl AppConfig {
    pub fn from_env() -> Self {
        use itconfig::get_env_or_default as env;

        Self {
            debug: env("DEBUG", false),
            host: env("HOST", "127.0.0.1".to_string()),
            port: env("PORT", 8800),
            workers: env("WORKERS", 8),
        }
    }
}
