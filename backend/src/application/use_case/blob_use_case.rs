use crate::application::service::blob::{Blob, BlobRepository, Metadata};
use chrono::{DateTime, Utc};
use derive_new::new;
use image::io::Reader;
use serde::Serialize;
use std::{convert::TryInto, io::Cursor, sync::Arc};
use ulid::Ulid;

#[derive(new, Debug, Clone)]
pub struct BlobUseCase<R: BlobRepository> {
    blob_repository: Arc<R>,
}

impl<R: BlobRepository> BlobUseCase<R> {
    pub async fn create_blob(
        &self,
        encoded: String,
        name: String,
        content_type: String,
    ) -> anyhow::Result<()> {
        let decoded = base64::decode(encoded)?;
        let img = Reader::new(Cursor::new(decoded.clone()))
            .with_guessed_format()?
            .decode()?;

        let byte_size: u32 = decoded.len().try_into()?;
        let metadata = Metadata::new(img.width(), img.height());

        let blob = Blob::new(
            Ulid::new().to_string(),
            decoded,
            name,
            content_type,
            byte_size,
            metadata,
            Utc::now(),
        );

        self.blob_repository.store(blob).await?;

        Ok(())
    }

    pub async fn find_blob(&self, id: String) -> anyhow::Result<BlobResponseDto> {
        let blob = self.blob_repository.find(id).await?;

        Ok(blob.into())
    }

    pub async fn find_all_blobs(&self) -> anyhow::Result<Vec<BlobResponseDto>> {
        let blobs = self.blob_repository.find_all().await?;

        Ok(blobs.into_iter().map(|blob| blob.into()).collect())
    }
}

#[derive(new, Debug, Serialize)]
pub struct BlobResponseDto {
    pub id: String,
    pub encoded: String,
    pub name: String,
    pub content_type: String,
    pub byte_size: u32,
    pub metadata: Metadata,
    pub created_at: DateTime<Utc>,
}

impl From<Blob> for BlobResponseDto {
    fn from(blob: Blob) -> Self {
        BlobResponseDto::new(
            blob.id,
            base64::encode(blob.data),
            blob.name,
            blob.content_type,
            blob.byte_size,
            blob.metadata,
            blob.created_at,
        )
    }
}
