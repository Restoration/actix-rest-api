


fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/hc").route(web::get().to(systems::health)))
        .service(web::resource("v1/news").route(web::get().to(user::user)));
}
