use crate::application::service::error::Error;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use ulid::Ulid;

#[async_trait]
pub trait BlobRepository {
    async fn store(&self, blob: Blob) -> Result<(), Error>;
    async fn delete(&self, id: String) -> Result<(), Error>;
    async fn find(&self, id: String) -> Result<Blob, Error>;
}

#[derive(Debug)]
pub struct Blob {
    pub id: String,
    pub data: Vec<u8>,
    pub name: String,
    pub content_type: String,
    pub byte_size: String,
    pub metadata: String,
    pub created_at: DateTime<Utc>,
}

impl Blob {
    fn new(
        data: Vec<u8>,
        name: String,
        content_type: String,
        byte_size: String,
        metadata: String,
    ) -> Self {
        Self {
            id: Ulid::new().to_string(),
            data,
            name,
            content_type,
            byte_size,
            metadata,
            created_at: Utc::now(),
        }
    }
}
