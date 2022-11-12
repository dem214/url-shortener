use actix_web::{App, HttpServer, web, middleware};
use services::shorten_storage::ShortenStorage;

mod applications;
mod services;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let shorten_storage = ShortenStorage::new("redis://localhost:6380");

    HttpServer::new(move || App::new()
            .service(
                web::scope("/api").configure(applications::api::config)
            )
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(shorten_storage.clone())))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
