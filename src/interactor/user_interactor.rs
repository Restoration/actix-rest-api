use async_trait::async_trait;
use crate::domain::{AppError, User, Users, UserId};
use crate::port::UserPort;

#[async_trait]
pub trait UserUseCase: Send + Sync {
    async fn find_user(&self, id: UserId) -> Result<User, AppError>;
    async fn find_users(&self) -> Result<Users, AppError>;
}

pub struct UserInteractor<P: UserPort> {
    pub user_port: P,
}

#[async_trait]
impl<P: UserPort> UserUseCase for UserInteractor<P> {
    async fn find_user(&self, id: UserId) -> Result<User, AppError> {
        self.user_port.find_user(id).await
    }

    async fn find_users(&self) -> Result<Users, AppError> {
        self.user_port.find_users().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockUserPort {
        users: Vec<User>,
        find_user_error: Option<AppError>,
        find_users_error: Option<AppError>,
    }

    impl MockUserPort {
        fn new(users: Vec<User>) -> Self {
            Self {
                users,
                find_user_error: None,
                find_users_error: None,
            }
        }

        fn with_find_user_error(mut self, err: AppError) -> Self {
            self.find_user_error = Some(err);
            self
        }

        fn with_find_users_error(mut self, err: AppError) -> Self {
            self.find_users_error = Some(err);
            self
        }
    }

    #[async_trait]
    impl UserPort for MockUserPort {
        async fn find_user(&self, id: UserId) -> Result<User, AppError> {
            if let Some(ref err) = self.find_user_error {
                return Err(match err {
                    AppError::NotFound => AppError::NotFound,
                    AppError::InternalServerError => AppError::InternalServerError,
                });
            }
            self.users
                .iter()
                .find(|u| u.id.0 == id.0)
                .cloned()
                .ok_or(AppError::NotFound)
        }

        async fn find_users(&self) -> Result<Users, AppError> {
            if let Some(ref err) = self.find_users_error {
                return Err(match err {
                    AppError::NotFound => AppError::NotFound,
                    AppError::InternalServerError => AppError::InternalServerError,
                });
            }
            Ok(self.users.clone())
        }
    }

    fn sample_users() -> Vec<User> {
        vec![
            User::new(UserId(1), "Alice"),
            User::new(UserId(2), "Bob"),
        ]
    }

    #[actix_web::test]
    async fn find_user_returns_matching_user() {
        let interactor = UserInteractor {
            user_port: MockUserPort::new(sample_users()),
        };
        let result = interactor.find_user(UserId(1)).await.unwrap();
        assert_eq!(result.id.0, 1);
        assert_eq!(result.name, "Alice");
    }

    #[actix_web::test]
    async fn find_user_returns_not_found_for_unknown_id() {
        let interactor = UserInteractor {
            user_port: MockUserPort::new(sample_users()),
        };
        let result = interactor.find_user(UserId(999)).await;
        assert!(result.is_err());
    }

    #[actix_web::test]
    async fn find_user_propagates_internal_error() {
        let mock = MockUserPort::new(vec![])
            .with_find_user_error(AppError::InternalServerError);
        let interactor = UserInteractor { user_port: mock };
        let result = interactor.find_user(UserId(1)).await;
        assert!(matches!(result, Err(AppError::InternalServerError)));
    }

    #[actix_web::test]
    async fn find_users_returns_all_users() {
        let interactor = UserInteractor {
            user_port: MockUserPort::new(sample_users()),
        };
        let result = interactor.find_users().await.unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "Alice");
        assert_eq!(result[1].name, "Bob");
    }

    #[actix_web::test]
    async fn find_users_returns_empty_vec() {
        let interactor = UserInteractor {
            user_port: MockUserPort::new(vec![]),
        };
        let result = interactor.find_users().await.unwrap();
        assert!(result.is_empty());
    }

    #[actix_web::test]
    async fn find_users_propagates_internal_error() {
        let mock = MockUserPort::new(vec![])
            .with_find_users_error(AppError::InternalServerError);
        let interactor = UserInteractor { user_port: mock };
        let result = interactor.find_users().await;
        assert!(matches!(result, Err(AppError::InternalServerError)));
    }
}
