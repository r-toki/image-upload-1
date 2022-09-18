use crate::application::service::storage_service::{Attachment, Blob, Error, StorageService};
use async_trait::async_trait;
use derive_new::new;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(new, Debug, Clone)]
pub struct StorageServiceImpl {
    pool: Arc<PgPool>,
}

#[async_trait]
impl StorageService for StorageServiceImpl {
    async fn store(&self, attachment: Attachment, blob: Blob) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(new, Debug, Serialize, Deserialize)]
pub struct AttachmentRow {
    pub id: i64,
    pub record_type: String,
    pub record_id: String,
    pub record_column_name: String,
    pub blob_id: String,
}

#[derive(new, Debug)]
pub struct BlobRow {
    pub id: i64,
    pub file_data: Vec<u8>,
    pub filename: String,
    pub content_type: String,
    pub byte_size: String,
    pub metadata: String,
}
