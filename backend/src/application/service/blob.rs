use async_trait::async_trait;
use chrono::{DateTime, Utc};
use derive_new::new;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Serialize)]
pub struct Blob {
    pub id: String,
    pub data: Vec<u8>,
    pub name: String,
    pub content_type: String,
    pub byte_size: i32,
    pub metadata: Metadata,
    pub created_at: DateTime<Utc>,
}

#[derive(new, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub width: i32,
    pub height: i32,
}

impl Blob {
    pub fn new(
        data: Vec<u8>,
        name: String,
        content_type: String,
        byte_size: i32,
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
