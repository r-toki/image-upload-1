use crate::application::service::blob_repository::{Blob, BlobRepository};
use crate::application::service::error::Error;
use async_trait::async_trait;
use derive_new::new;
use sqlx::{query, query_as, PgPool};
use std::sync::Arc;

#[derive(new, Debug, Clone)]
pub struct BlobRepositoryImpl {
    pool: Arc<PgPool>,
}

#[async_trait]
impl BlobRepository for BlobRepositoryImpl {
    async fn store(&self, blob: Blob) -> Result<(), Error> {
        query!(
            r#"
insert into blobs
(id, data, name, content_type, byte_size, metadata, created_at)
values
($1, $2, $3, $4, $5, $6, $7)
            "#,
            blob.id,
            blob.data,
            blob.name,
            blob.content_type,
            blob.byte_size,
            blob.metadata,
            blob.created_at,
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: String) -> Result<(), Error> {
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

    async fn find(&self, id: String) -> Result<Blob, Error> {
        let blob = query_as!(
            Blob,
            r#"
select * from blobs
where id = $1
            "#,
            id
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(blob)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::Unknown(format!("{:?}", e))
    }
}
