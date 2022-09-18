use async_trait::async_trait;
use chrono::{DateTime, Utc};
use derive_new::new;

#[async_trait]
pub trait StorageService {
    async fn insert(&self, attachment: Attachment, blob: Blob) -> Result<(), Error>;
    // async fn delete(&self, attachment: Attachment) -> Result<(), Error>;
    // async fn find(&self, attachment: Attachment) -> Result<Blob, Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{}", .0)]
    Unknown(String),
}

#[derive(new, Debug)]
pub struct Attachment {
    pub record_type: String,
    pub record_id: String,
    pub record_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(new, Debug)]
pub struct Blob {
    pub file_data: Vec<u8>,
    pub filename: String,
    pub content_type: String,
    pub byte_size: String,
    pub metadata: String,
    pub created_at: DateTime<Utc>,
}
