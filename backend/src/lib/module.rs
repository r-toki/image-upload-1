use crate::application::use_case::blob_use_case::BlobUseCase;
use crate::infrastructure::blob_repository_impl::BlobRepositoryImpl;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Module {
    pub blob_use_case: BlobUseCase<BlobRepositoryImpl>,
}

impl Module {
    pub fn new(pool: PgPool) -> Self {
        let pool = Arc::new(pool);

        let blob_repository = Arc::new(BlobRepositoryImpl::new(pool.clone()));

        let blob_use_case = BlobUseCase::new(blob_repository.clone());

        Self { blob_use_case }
    }
}
