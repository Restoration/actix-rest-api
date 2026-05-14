use crate::presenter::health_check_presenter::health_check;
use crate::presenter::user_presenter::{get_user, get_users};

use actix_web::web;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/hc").route(web::get().to(health_check)))
        .service(web::resource("/v1/users").route(web::get().to(get_users)))
        .service(web::resource("/v1/users/{id}").route(web::get().to(get_user)));
}
