use async_trait::async_trait;
use crate::domain::user::{User,  Users};
use crate::domain::error::Error;
use crate::port::user_port::UserPort;

#[derive(Debug, Copy, Clone)]
pub struct UserRepository;

#[async_trait(?Send)]
impl UserPort for UserRepository {
    async fn find_user() -> Result<User, Error> {}
    async fn find_users() -> Result<Users, Error> {}
}
