use actix_web::{error, web, Responder};
use serde::{Deserialize, Serialize};

use crate::services::shorten_storage::ShortenStorage;

#[derive(Serialize, Deserialize)]
struct PostShortenPayload {
    link: String,
}

#[derive(Serialize)]
struct GetShortenResponse {
    code: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .name("shorten")
            .route(web::post().to(post)),
    )
    .service(
        web::resource("/{code}")
            .name("shorten_details")
            .route(web::get().to(get)),
    );
}

async fn get(
    path: web::Path<String>,
    shorten_storage: web::Data<ShortenStorage>,
) -> actix_web::Result<impl Responder> {
    let code = path.into_inner();
    let link = shorten_storage
        .get(&code)
        .await
        .ok_or_else(|| error::ErrorNotFound("invalid code"))?;
    Ok(web::Json(PostShortenPayload { link }))
}

async fn post(
    payload: web::Json<PostShortenPayload>,
    shorten_storage: web::Data<ShortenStorage>,
) -> actix_web::Result<impl Responder> {
    let link = payload.into_inner().link;
    let code = shorten_storage
        .save(link)
        .await
        .map_err(error::ErrorInternalServerError)?;
    Ok(web::Json(GetShortenResponse { code }))
}
