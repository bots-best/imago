//! The web module of the imago service which handles
//! everything related to the json api.

mod routes;

use crate::{models::Id, AppConfig, util::{ready, Ready}};
use eyre::{WrapErr, Result};
use actix_web::{App, HttpServer, Responder, Error, HttpResponse};

/// Launches the actix web application.
pub async fn launch_app(cfg: AppConfig) -> Result<()> {
    HttpServer::new(move || {
        App::new().configure(routes::init)
    })
    .workers(cfg.workers)
    .bind(format!("{}:{}", cfg.host, cfg.port))
    .wrap_err("failed to bind application to address")?
    .run()
    .await
    .wrap_err("failed to start web server")
}

impl Responder for Id {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &actix_web::HttpRequest) -> Self::Future {
        let body = serde_json::json!({ "id": self.id });

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))

    }
}
