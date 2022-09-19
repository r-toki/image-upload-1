use crate::application::service::blob::Blob;
use crate::lib::module::Module;
use actix_web::{
    error::ErrorInternalServerError,
    get, post,
    web::{Data, Json, Path, ServiceConfig},
};
use serde::{Deserialize, Serialize};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(create);
    cfg.service(blob::index);
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateForm {
    encoded_data: String,
    name: String,
    content_type: String,
}

#[post("/blobs")]
async fn create(module: Data<Module>, form: Json<CreateForm>) -> actix_web::Result<Json<()>> {
    module
        .blob_use_case
        .create_blob(
            form.encoded_data.to_owned(),
            form.name.to_owned(),
            form.content_type.to_owned(),
        )
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(Json(()))
}

mod blob {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct BlobPath {
        pub blob_id: String,
    }

    #[get("/blobs/{blob_id}")]
    async fn index(module: Data<Module>, path: Path<BlobPath>) -> actix_web::Result<Json<Blob>> {
        let blob = module
            .blob_use_case
            .find_blob(path.blob_id.to_string())
            .await
            .map_err(ErrorInternalServerError)?;

        Ok(Json(blob))
    }
}
