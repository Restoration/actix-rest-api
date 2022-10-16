use crate::container::container::Container;
use crate::domain::user::{User, UserId};
use crate::interactor::user_interactor::UserInteractor;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub async fn User(data: web::Data<Container>) -> impl Responder {
    let user = UserInteractor::find_user(data.user_port).await;

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
    id: UserId,
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
