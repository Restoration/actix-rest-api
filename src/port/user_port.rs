use crate::domain::error::Error;
use crate::domain::user::{User, Users, UserId};
use async_trait::async_trait;
use sea_orm::DatabaseConnection;

#[async_trait(?Send)]
pub trait UserPort {
    async fn find_user(db: DatabaseConnection, id: UserId) -> Result<User, Error>;
    async fn find_users(db: DatabaseConnection,) -> Result<Users, Error>;
}
