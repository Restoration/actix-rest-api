use crate::container::Container;
use crate::domain::{User, UserId};
use actix_web::{web, HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};

pub async fn get_user(
    data: web::Data<Container>,
    path: web::Path<u32>,
) -> impl Responder {
    let id = UserId(path.into_inner());
    match data.user_use_case.find_user(id).await {
        Ok(user) => HttpResponse::Ok().json(UserResponse::from(user)),
        Err(e) => e.error_response(),
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
        Err(e) => e.error_response(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{AppError, Users};
    use crate::interactor::UserUseCase;
    use actix_web::test;
    use actix_web::App;
    use async_trait::async_trait;
    use std::sync::Arc;

    struct MockUseCase {
        users: Vec<User>,
        find_user_error: Option<AppError>,
        find_users_error: Option<AppError>,
    }

    #[async_trait]
    impl UserUseCase for MockUseCase {
        async fn find_user(&self, id: UserId) -> Result<User, AppError> {
            if let Some(ref err) = self.find_user_error {
                return Err(err.clone());
            }
            self.users
                .iter()
                .find(|u| u.id.0 == id.0)
                .cloned()
                .ok_or(AppError::NotFound)
        }

        async fn find_users(&self) -> Result<Users, AppError> {
            if let Some(ref err) = self.find_users_error {
                return Err(err.clone());
            }
            Ok(self.users.clone())
        }
    }

    fn mock_container(users: Vec<User>) -> web::Data<Container> {
        web::Data::new(Container {
            user_use_case: Arc::new(MockUseCase {
                users,
                find_user_error: None,
                find_users_error: None,
            }),
        })
    }

    fn mock_container_with_error(find_user_err: Option<AppError>, find_users_err: Option<AppError>) -> web::Data<Container> {
        web::Data::new(Container {
            user_use_case: Arc::new(MockUseCase {
                users: vec![],
                find_user_error: find_user_err,
                find_users_error: find_users_err,
            }),
        })
    }

    fn sample_users() -> Vec<User> {
        vec![
            User::new(UserId(1), "Alice"),
            User::new(UserId(2), "Bob"),
        ]
    }

    #[actix_web::test]
    async fn get_users_returns_200_with_users() {
        let container = mock_container(sample_users());
        let app = test::init_service(
            App::new()
                .app_data(container)
                .route("/v1/users", web::get().to(get_users)),
        )
        .await;

        let req = test::TestRequest::get().uri("/v1/users").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);

        let body: Vec<UserResponse> = test::read_body_json(resp).await;
        assert_eq!(body.len(), 2);
        assert_eq!(body[0].id, 1);
        assert_eq!(body[0].name, "Alice");
        assert_eq!(body[1].id, 2);
        assert_eq!(body[1].name, "Bob");
    }

    #[actix_web::test]
    async fn get_users_returns_200_with_empty_list() {
        let container = mock_container(vec![]);
        let app = test::init_service(
            App::new()
                .app_data(container)
                .route("/v1/users", web::get().to(get_users)),
        )
        .await;

        let req = test::TestRequest::get().uri("/v1/users").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);

        let body: Vec<UserResponse> = test::read_body_json(resp).await;
        assert!(body.is_empty());
    }

    #[actix_web::test]
    async fn get_users_returns_500_on_internal_error() {
        let container = mock_container_with_error(None, Some(AppError::InternalServerError));
        let app = test::init_service(
            App::new()
                .app_data(container)
                .route("/v1/users", web::get().to(get_users)),
        )
        .await;

        let req = test::TestRequest::get().uri("/v1/users").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn get_user_returns_200_with_user() {
        let container = mock_container(sample_users());
        let app = test::init_service(
            App::new()
                .app_data(container)
                .route("/v1/users/{id}", web::get().to(get_user)),
        )
        .await;

        let req = test::TestRequest::get().uri("/v1/users/1").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);

        let body: UserResponse = test::read_body_json(resp).await;
        assert_eq!(body.id, 1);
        assert_eq!(body.name, "Alice");
    }

    #[actix_web::test]
    async fn get_user_returns_404_for_unknown_id() {
        let container = mock_container(sample_users());
        let app = test::init_service(
            App::new()
                .app_data(container)
                .route("/v1/users/{id}", web::get().to(get_user)),
        )
        .await;

        let req = test::TestRequest::get().uri("/v1/users/999").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn get_user_returns_500_on_internal_error() {
        let container = mock_container_with_error(Some(AppError::InternalServerError), None);
        let app = test::init_service(
            App::new()
                .app_data(container)
                .route("/v1/users/{id}", web::get().to(get_user)),
        )
        .await;

        let req = test::TestRequest::get().uri("/v1/users/1").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 500);
    }
}
