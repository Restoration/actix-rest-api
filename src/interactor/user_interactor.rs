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
