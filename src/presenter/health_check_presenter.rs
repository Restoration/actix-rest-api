use actix_web::{web, HttpResponse, Responder};
use crate::container::Container;

pub async fn health_check(data: web::Data<Container>) -> impl Responder {
    HttpResponse::Ok()
}


