use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .json("{\"message\":\"Hello World!!\"}")
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/hc", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
