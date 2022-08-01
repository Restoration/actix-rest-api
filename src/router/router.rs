use crate::presenter::health_check_presenter::HealthCheck;
use crate::presenter::user_presenter::User;

use actix_web::web;
pub fn Routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/hc").route(web::get().to(HealthCheck)))
        .service(web::resource("v1/news").route(web::get().to(User)));
}
