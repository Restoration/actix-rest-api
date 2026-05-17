use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use super::*;

    #[actix_web::test]
    async fn health_check_returns_200() {
        let app = test::init_service(
            App::new().route("/hc", web::get().to(health_check)),
        )
        .await;

        let req = test::TestRequest::get().uri("/hc").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);
    }
}
