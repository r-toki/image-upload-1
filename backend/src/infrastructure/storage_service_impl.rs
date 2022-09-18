use crate::application::service::storage_service::{Attachment, Blob, Error, StorageService};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use derive_new::new;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool};
use std::sync::Arc;

#[derive(new, Debug, Clone)]
pub struct StorageServiceImpl {
    pool: Arc<PgPool>,
}

#[async_trait]
impl StorageService for StorageServiceImpl {
    async fn insert(&self, attachment: Attachment, blob: Blob) -> Result<(), Error> {
        let blob = query_as!(
            BlobRow,
            r#"
insert into blobs
(file_data, filename, content_type, byte_size, metadata, created_at)
values
($1, $2, $3, $4, $5, current_timestamp)
returning *
            "#,
            blob.file_data,
            blob.filename,
            blob.content_type,
            blob.byte_size,
            blob.metadata,
        )
        .fetch_one(&*self.pool)
        .await?;

        query!(
            r#"
insert into attachments
(record_type, record_id, record_name, created_at, blob_id)
values
($1, $2, $3, current_timestamp, $4)
            "#,
            attachment.record_type,
            attachment.record_id,
            attachment.record_name,
            blob.id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::Unknown(format!("{:?}", e))
    }
}

#[derive(new, Debug, Serialize, Deserialize)]
struct AttachmentRow {
    pub id: i32,
    pub record_type: String,
    pub record_id: String,
    pub record_name: String,
    pub created_at: DateTime<Utc>,
    pub blob_id: String,
}

#[derive(new, Debug, Serialize, Deserialize)]
struct BlobRow {
    pub id: i32,
    pub file_data: Vec<u8>,
    pub filename: String,
    pub content_type: String,
    pub byte_size: String,
    pub metadata: String,
    pub created_at: DateTime<Utc>,
}
