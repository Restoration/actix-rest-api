use crate::domain::user::Users;
use crate::domain::error::Error;
use crate::port::user_port::UserPort;

pub async fn execute(user_port: impl UserPort) -> Result<Users, Error> {
    user_port.find_users().await?
}
