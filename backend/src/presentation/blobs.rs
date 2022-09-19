use actix_web::{
    post,
    web::{Json, ServiceConfig},
    Responder,
};
use serde::Deserialize;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(create);
}

#[derive(Debug, Deserialize)]
struct CreateForm {
    encoded_data: String,
    name: String,
    content_type: String,
    metadata: Metadata,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    width: i16,
    height: i16,
}

#[post("/blobs")]
async fn create(form: Json<CreateForm>) -> impl Responder {
    Json(())
}
