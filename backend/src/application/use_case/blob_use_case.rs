use crate::application::service::blob::{Blob, BlobRepository, Metadata};
use derive_new::new;
use image::io::Reader;
use std::{convert::TryInto, io::Cursor, sync::Arc};

#[derive(new, Debug, Clone)]
pub struct BlobUseCase<R: BlobRepository> {
    blob_repository: Arc<R>,
}

impl<R: BlobRepository> BlobUseCase<R> {
    pub async fn create_blob(
        &self,
        encoded_data: String,
        name: String,
        content_type: String,
    ) -> anyhow::Result<()> {
        let decoded = base64::decode(encoded_data)?;
        let img = Reader::new(Cursor::new(decoded.clone()))
            .with_guessed_format()?
            .decode()?;

        let byte_size: i32 = decoded.len().try_into()?;
        let metadata = Metadata::new(img.width().try_into()?, img.height().try_into()?);

        let blob = Blob::new(decoded, name, content_type, byte_size, metadata);

        self.blob_repository.store(blob).await?;

        Ok(())
    }

    pub async fn find_blob(&self, id: String) -> anyhow::Result<Blob> {
        let blob = self.blob_repository.find(id).await?;

        Ok(blob)
    }
}
