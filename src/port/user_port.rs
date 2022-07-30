use create::domain::user::{UserIds, User, Users}
use crate::error::Error;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait UserPort {
    async fn find_user() -> Result<User, Error>
    async fn find_users() -> Result<Users, Error>
}

