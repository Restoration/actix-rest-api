mod domain;
mod repository;
mod port;
mod presenter;
mod usecase;
mod router;
mod container;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(Container {
                user_port: UserRepository {},
            })
            .wrap(Logger::default())
            .configure(routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
