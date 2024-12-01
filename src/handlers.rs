use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("¡Hola, mundo!")
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
