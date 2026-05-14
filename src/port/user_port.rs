use crate::domain::{AppError, User, Users, UserId};
use async_trait::async_trait;

#[async_trait]
pub trait UserPort: Send + Sync {
    async fn find_user(&self, id: UserId) -> Result<User, AppError>;
    async fn find_users(&self) -> Result<Users, AppError>;
}
