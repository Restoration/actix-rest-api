use crate::domain::user::User;
use crate::error::Error;
use crate::port::user_port::UserPort;

pub async fn execute(user_port: impl NewsPort) -> Result<Users, Error> {
   user_port.find_users().await?;
}
