use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use crate::domain::user::{User, Users, UserId};
use crate::domain::error::Error;
use crate::port::user_port::UserPort;


// pub struct UserUseCase {
//     db: DatabaseConnection,
//     user_port: dyn UserPort,
// }


#[async_trait(?Send)]
pub trait UserUseCase {
    async fn find_user(db: DatabaseConnection, id: UserId) -> Result<User, Error>;
    async fn find_users(db: DatabaseConnection,) -> Result<Users, Error>;
}

#[derive(Debug, Copy, Clone)]
pub struct UserInteractor;

#[async_trait(?Send)]
impl UserInteractor for dyn UserUseCase {
    async fn find_user(id: UserId) -> Result<User, Error> {
        return Self::user_port.find_user(Self::db, id).await?
    }
    async fn find_users() -> Result<Users, Error> {
        return Self::user_port.find_users(Self::db).await?
    }
}
