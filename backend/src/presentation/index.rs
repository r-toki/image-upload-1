use actix_web::{
    get,
    web::{Json, ServiceConfig},
    Responder,
};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
}

#[get("/")]
async fn index() -> impl Responder {
    Json("Hello World")
}
