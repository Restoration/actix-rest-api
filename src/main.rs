mod config;
mod container;
mod dao;
mod domain;
mod interactor;
mod port;
mod presenter;
mod repository;
mod router;

use std::sync::Arc;
use crate::config::Config;
use crate::container::Container;
use crate::interactor::UserInteractor;
use crate::repository::UserRepository;
use crate::router::routes;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sea_orm::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let config = envy::from_env::<Config>().expect("Failed to load config from env");
    log::info!("Server starting on port {}", config.port);

    let port = config.port;

    let db = Database::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    let user_repository = UserRepository::new(db);
    let user_interactor = UserInteractor::new(user_repository);

    let container = web::Data::new(Container {
        user_use_case: Arc::new(user_interactor),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(container.clone())
            .wrap(Logger::default())
            .configure(routes)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
