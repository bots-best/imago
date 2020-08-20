//! All routes for the Http api.

use crate::{error, models::{Id, ImageUploadRequest}};
use actix_web::{post, web::{self, Bytes}, HttpResponse};
use eyre::Result;
use std::thread_local;
use awc::{http::header::HeaderValue, Client};
use image::ImageFormat;

/// Executes the given closure with the thread local http client.
fn with_http_client<T, F: FnOnce(&Client) -> T>(cb: F) -> T {
    thread_local! {
        static CLIENT: Client = Client::new();
    }
    CLIENT.with(cb)
}

async fn fetch_image(url: String) -> Result<(Bytes, Option<HeaderValue>), HttpResponse> {
    let mut raw_body = with_http_client(|client| {
        client.get(url).send()
    }).await.map_err(|err| match err {
        actix_web::client::SendRequestError::Url(_) => error!("invalid url provided", BadRequest),
        actix_web::client::SendRequestError::Connect(_) => error!("failed to download image from url", BadRequest),
        err => {
            log::error!("failed to get image: {}", err);
            error!("internal server error, sorry", InternalServerError)
        },
    })?;
    let header = raw_body.headers().get("Content-Type").cloned();
    raw_body.body().await.map(|body| (body, header)).map_err(|err| {
        log::error!("failed to get raw body of image: {}", err);
        error!("internal server error, sorry", InternalServerError)
    })
}

fn image_format_for_type(content: &str) -> Result<ImageFormat, HttpResponse> {
    match content {
        "image/png" => Ok(ImageFormat::Png),
        "image/jpeg" => Ok(ImageFormat::Jpeg),
        _ => Err(error!("image host returned invalid content", BadRequest)),
    }
}

#[post("/upload")]
async fn upload(req: web::Json<ImageUploadRequest>) -> Result<Id, HttpResponse> {

    let ImageUploadRequest { url, formats, sizes } = req.into_inner();
    let (image, content_type) = fetch_image(url).await?;

    let content_type = content_type.ok_or_else(|| error!("image url returned a bad request", BadRequest))?;
    let content_type = content_type.to_str().map_err(|err| {
        log::error!("failed to convert content type to string: {}", err);
        error!("image host returned bad request", BadRequest)
    })?;
    let image_format = image_format_for_type(content_type);

    Ok(uuid::Uuid::nil().into())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(upload);
}
