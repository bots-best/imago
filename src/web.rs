//! The web module of the imago service which handles
//! everything related to the json api.

use crate::AppConfig;
use eyre::{WrapErr, Result};
use actix_web::{App, HttpServer};

/// Launches the actix web application.
pub async fn launch_app(cfg: AppConfig) -> Result<()> {
    HttpServer::new(move || {
        App::new()
    })
    .workers(cfg.workers)
    .bind(format!("{}:{}", cfg.host, cfg.port))
    .wrap_err("failed to bind application to address")?
    .run()
    .await
    .wrap_err("failed to start web server")
}
