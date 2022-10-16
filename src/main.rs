mod container;
mod domain;
mod port;
mod presenter;
mod repository;
mod router;
mod interactor;
mod dao;
use crate::container::container::Container;
use crate::repository::user_repository::UserRepository;
use crate::router::router::Routes;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use sea_orm::{Database, DatabaseConnection};

#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
    let db: DatabaseConnection = Database::connect("protocol://username:password@host/database").await?;
    HttpServer::new(|| {
        App::new()
            .data(Container {
                user_port: UserRepository {},
            })
            .wrap(Logger::default())
            .configure(Routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
