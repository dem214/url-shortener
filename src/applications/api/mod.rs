use actix_web::web;

mod shorten;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/shorten").configure(shorten::config));
}
