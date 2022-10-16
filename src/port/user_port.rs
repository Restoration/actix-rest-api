use crate::domain::error::Error;
use crate::domain::user::{User, Users, UserId};
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait UserPort {
    async fn find_user(id: UserId) -> Result<User, Error>;
    async fn find_users() -> Result<Users, Error>;
}
