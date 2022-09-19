use crate::application::service::blob::{Blob, BlobRepository, Metadata};
use async_trait::async_trait;
use derive_new::new;
use sqlx::{query, PgPool};
use std::{convert::TryInto, sync::Arc};

#[derive(new, Debug, Clone)]
pub struct BlobRepositoryImpl {
    pool: Arc<PgPool>,
}

#[async_trait]
impl BlobRepository for BlobRepositoryImpl {
    async fn store(&self, blob: Blob) -> anyhow::Result<()> {
        let byte_size: i32 = blob.byte_size.try_into()?;
        let metadata = serde_json::to_string(&blob.metadata)?;

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
            byte_size,
            metadata,
            blob.created_at,
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
        let r = query!(
            r#"
select * from blobs
where id = $1
            "#,
            id
        )
        .fetch_one(&*self.pool)
        .await?;

        let metadata = serde_json::from_str::<Metadata>(&r.metadata)?;
        let blob = Blob::new(
            r.id,
            r.data,
            r.name,
            r.content_type,
            r.byte_size as u32,
            metadata,
            r.created_at,
        );

        Ok(blob)
    }
}
