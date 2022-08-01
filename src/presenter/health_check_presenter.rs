use crate::container::Container;
use actix_web::{web, HttpResponse, Responder};

pub async fn HealthCheck(data: web::Data<Container>) -> impl Responder {
    HttpResponse::Ok()
}
