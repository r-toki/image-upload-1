use crate::application::use_case::blob_use_case::BlobResponseDto;
use crate::lib::module::Module;
use actix_web::{
    error::ErrorInternalServerError,
    get, post,
    web::{Data, Json, Path, ServiceConfig},
};
use serde::{Deserialize, Serialize};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(create);
    cfg.service(blob::index);
}

#[get("/blobs")]
async fn index(module: Data<Module>) -> actix_web::Result<Json<Vec<BlobResponseDto>>> {
    module
        .blob_use_case
        .find_all_blobs()
        .await
        .map(Json)
        .map_err(ErrorInternalServerError)
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateForm {
    encoded: String,
    name: String,
    content_type: String,
}

#[post("/blobs")]
async fn create(module: Data<Module>, form: Json<CreateForm>) -> actix_web::Result<Json<()>> {
    module
        .blob_use_case
        .create_blob(
            form.encoded.to_owned(),
            form.name.to_owned(),
            form.content_type.to_owned(),
        )
        .await
        .map(Json)
        .map_err(ErrorInternalServerError)
}

mod blob {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct BlobPath {
        pub blob_id: String,
    }

    #[get("/blobs/{blob_id}")]
    async fn index(
        module: Data<Module>,
        path: Path<BlobPath>,
    ) -> actix_web::Result<Json<BlobResponseDto>> {
        module
            .blob_use_case
            .find_blob(path.blob_id.to_owned())
            .await
            .map(Json)
            .map_err(ErrorInternalServerError)
    }
}
