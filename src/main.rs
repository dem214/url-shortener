use actix_web::{App, HttpServer};

mod applications;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().configure(applications::api::config))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
