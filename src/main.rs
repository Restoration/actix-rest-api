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
use crate::config::config::Config;
use crate::container::container::Container;
use crate::interactor::user_interactor::UserInteractor;
use crate::repository::user_repository::UserRepository;
use crate::router::router::routes;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sea_orm::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config = envy::from_env::<Config>().expect("Failed to load config from env");
    println!("{:#?}", config);

    let db = Database::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    let user_repository = UserRepository { db };
    let user_interactor = UserInteractor {
        user_port: user_repository,
    };

    let container = web::Data::new(Container {
        user_use_case: Arc::new(user_interactor),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(container.clone())
            .wrap(Logger::default())
            .configure(routes)
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}
