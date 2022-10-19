use actix_web::{web, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api")
            .route(web::get().to(get))
            .route(web::post().to(post)),
    );
}

async fn get() -> impl Responder {
    HttpResponse::Ok().body("get")
}

async fn post() -> impl Responder {
    HttpResponse::Ok().body("post")
}