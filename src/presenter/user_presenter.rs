use crate::container::container::Container;
use crate::domain::user::{User, UserId};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub async fn get_user(
    data: web::Data<Container>,
    path: web::Path<u32>,
) -> impl Responder {
    let id = UserId(path.into_inner());
    match data.user_use_case.find_user(id).await {
        Ok(user) => HttpResponse::Ok().json(UserResponse::from(user)),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

pub async fn get_users(data: web::Data<Container>) -> impl Responder {
    match data.user_use_case.find_users().await {
        Ok(users) => HttpResponse::Ok().json(
            users
                .into_iter()
                .map(UserResponse::from)
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

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id.0,
            name: user.name,
        }
    }
}
