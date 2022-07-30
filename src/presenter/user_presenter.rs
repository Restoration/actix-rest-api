use crate::domain::user::User;
use actix_web::{web, HttpResponse, Responder};
use crate::usecase::user_usecase;
use crate::container::Container;
use serde::{Deserialize, Serialize};


pub async fn user(data: web::Data<Container>) -> impl Responder {
    let user = user_usecase::execute(data.user_port).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(
            user.into_iter()
                .map(|user| UserResponse::from(user))
                .collect::<Vec<UserResponse>>(),
        ),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct UserResponse {
    id: u32,
    name: String,
}

impl UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            name: user.name,
        }
    }
}
