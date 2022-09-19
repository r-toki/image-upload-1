use actix_web::{
    post,
    web::{Json, ServiceConfig},
    Responder,
};
use serde::{Deserialize, Serialize};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(create);
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateForm {
    encoded_data: String,
    name: String,
    content_type: String,
}

#[post("/blobs")]
async fn create(form: Json<CreateForm>) -> impl Responder {
    Json(form.0)
}
