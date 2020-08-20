//! All routes for the Http api.

use crate::models::{Id, ImageUploadRequest};
use actix_web::{post, web, HttpResponse};
use eyre::Result;

#[post("/upload")]
async fn upload(req: web::Json<ImageUploadRequest>) -> Result<Id, HttpResponse> {
    Ok(uuid::Uuid::nil().into())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(upload);
}
