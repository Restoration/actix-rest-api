use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use crate::domain::user::{User, Users, UserId};
use crate::domain::error::Error;
use crate::port::user_port::UserPort;


#[derive(Debug, Copy, Clone)]
pub struct UserUseCase {
    db: DatabaseConnection,
    user_port: dyn UserPort,
}
pub struct UserInteractor;

#[async_trait(?Send)]
impl UserInteractor for UserUseCase {
    async fn find_user(id: UserId) -> Result<User, Error> {
        return user_port.find_user(db, id).await?
    }
    async fn find_users() -> Result<Users, Error> {
        return user_port.find_users(db).await?
    }
}
