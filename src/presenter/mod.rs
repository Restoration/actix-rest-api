
pub mod user_presenter;
pub mod health_check_presenter;

pub mod user;

#[actix_rt::main]
pub async fn build() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(Container {
                news_port: NewsGateway {},
            })
            .wrap(Logger::default())
            .configure(routes)
    })
    .bind("0.0.0.0:3333")?
    .run()
    .await
}

fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/healthz").route(web::get().to(systems::healthz)))
        .service(web::resource("v1/news").route(web::get().to(news::news)));
}

pub struct Container {
    user_port: UserRepository,
}
