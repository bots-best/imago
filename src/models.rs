//! All models of the imago web server.

use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id {
    pub id: Uuid,
}

impl Into<Uuid> for Id {
    fn into(self) -> Uuid {
        self.id
    }
}

impl From<Uuid> for Id {
    fn from(id: Uuid) -> Self {
        Self { id }
    }
}

#[derive(Debug, Deserialize)]
pub struct ImageUploadRequest {
    pub url: Url,
    pub formats: Vec<ImageFormat>,
    pub sizes: Vec<ImageSize>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageFormat {
    Png,
    Jpeg,
}

#[derive(Debug, Deserialize)]
pub struct ImageSize {
    pub height: u32,
    pub width: u32,
}
