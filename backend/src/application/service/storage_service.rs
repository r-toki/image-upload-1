use async_trait::async_trait;
use derive_new::new;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{}", .0)]
    Unknown(String),
}

#[derive(new, Debug)]
pub struct Attachment {
    pub record_type: String,
    pub record_id: String,
    pub record_column_name: String,
}

#[derive(new, Debug)]
pub struct Blob {
    pub encoded: Vec<u8>,
    pub filename: String,
    pub content_type: String,
    pub byte_size: String,
    pub metadata: String,
}

#[async_trait]
pub trait StorageService {
    async fn store(&self, attachment: Attachment, blob: Blob) -> Result<(), Error>;
    // async fn delete(&self, attachment: Attachment) -> Result<(), Error>;
    // async fn find(&self, attachment: Attachment) -> Result<Blob, Error>;
}
