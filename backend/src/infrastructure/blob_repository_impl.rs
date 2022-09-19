use crate::application::service::blob::{Blob, BlobRepository};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use derive_new::new;
use sqlx::{query, query_as, PgPool};
use std::sync::Arc;

#[derive(new, Debug, Clone)]
pub struct BlobRepositoryImpl {
    pool: Arc<PgPool>,
}

#[async_trait]
impl BlobRepository for BlobRepositoryImpl {
    async fn store(&self, blob: Blob) -> anyhow::Result<()> {
        let blob_row: BlobRow = blob.into();
        query!(
            r#"
insert into blobs
(id, data, name, content_type, byte_size, metadata, created_at)
values
($1, $2, $3, $4, $5, $6, $7)
            "#,
            blob_row.id,
            blob_row.data,
            blob_row.name,
            blob_row.content_type,
            blob_row.byte_size,
            blob_row.metadata,
            blob_row.created_at,
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: String) -> anyhow::Result<()> {
        query!(
            r#"
delete from blobs
where id = $1
            "#,
            id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    async fn find(&self, id: String) -> anyhow::Result<Blob> {
        let blob_row = query_as!(
            BlobRow,
            r#"
select * from blobs
where id = $1
            "#,
            id
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(blob_row.into())
    }
}

#[derive(Debug)]
struct BlobRow {
    pub id: String,
    pub data: Vec<u8>,
    pub name: String,
    pub content_type: String,
    pub byte_size: String,
    pub metadata: String,
    pub created_at: DateTime<Utc>,
}

impl From<BlobRow> for Blob {
    fn from(blob_row: BlobRow) -> Self {
        Self {
            id: blob_row.id,
            data: blob_row.data,
            name: blob_row.name,
            content_type: blob_row.content_type,
            byte_size: blob_row.byte_size,
            metadata: serde_json::from_str(&blob_row.metadata).unwrap(),
            created_at: blob_row.created_at,
        }
    }
}

impl From<Blob> for BlobRow {
    fn from(blob: Blob) -> Self {
        Self {
            id: blob.id,
            data: blob.data,
            name: blob.name,
            content_type: blob.content_type,
            byte_size: blob.byte_size,
            metadata: serde_json::to_string(&blob.metadata).unwrap(),
            created_at: blob.created_at,
        }
    }
}
