use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug)]
pub struct Blob {
    pub id: String,
    pub data: Vec<u8>,
    pub name: String,
    pub content_type: String,
    pub byte_size: String,
    pub metadata: Metadata,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub width: i16,
    pub height: i16,
}

impl Blob {
    fn new(
        data: Vec<u8>,
        name: String,
        content_type: String,
        byte_size: String,
        metadata: Metadata,
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

#[async_trait]
pub trait BlobRepository {
    async fn store(&self, blob: Blob) -> anyhow::Result<()>;
    async fn delete(&self, id: String) -> anyhow::Result<()>;
    async fn find(&self, id: String) -> anyhow::Result<Blob>;
}
