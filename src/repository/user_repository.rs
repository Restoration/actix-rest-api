use async_trait::async_trait;
use create::domain::users::{UserId, UserIds, Users, Users};
use create::error::Error;
use create::port::users_port::UserPort;

#[derive(Debug, Copy, Clone)]
pub struct UserRepository;

#[async_trait(?Send)]
impl UserPort for UserRepository {
    async fn find_user() -> Result<User, Error> {}
    async fn find_users() -> Result<User, Error> {}
}
