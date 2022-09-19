use async_trait::async_trait;
use chrono::{DateTime, Utc};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(new, Debug, Serialize, Deserialize)]
pub struct Blob {
    pub id: String,
    pub data: Vec<u8>,
    pub name: String,
    pub content_type: String,
    pub byte_size: u32,
    pub metadata: Metadata,
    pub created_at: DateTime<Utc>,
}

#[derive(new, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub width: u32,
    pub height: u32,
}

#[async_trait]
pub trait BlobRepository {
    async fn store(&self, blob: Blob) -> anyhow::Result<()>;
    async fn delete(&self, id: String) -> anyhow::Result<()>;
    async fn find(&self, id: String) -> anyhow::Result<Blob>;
    async fn find_all(&self) -> anyhow::Result<Vec<Blob>>;
}
