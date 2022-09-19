use crate::application::service::blob::BlobRepository;
use derive_new::new;
use std::sync::Arc;

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
        Ok(())
    }
}
