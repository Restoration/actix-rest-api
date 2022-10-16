use async_trait::async_trait;
use sea_orm::{DatabaseConnection};
use crate::domain::user::{User,  Users, UserId};
use crate::domain::error::Error;
use crate::dao::user::Model;
use crate::port::user_port::UserPort;

#[derive(Debug, Copy, Clone)]
pub struct UserRepository;

#[async_trait(?Send)]
impl UserPort for UserRepository {
    async fn find_user(db: DatabaseConnection, id: UserId) -> Result<User, Error> {
        let user: Option<Model> = User::find_by_id(id).one(db).await?;
        let user: Model = user.unwrap();
        return user
    }
    async fn find_users(db: DatabaseConnection) -> Result<Users, Error> {
        let users: Vec<Model> = Users::find().all(db).await?;
        return users
    }
}
