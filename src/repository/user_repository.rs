use async_trait::async_trait;
use log::error;
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::dao::user::Entity as UserEntity;
use crate::domain::{AppError, User, Users, UserId};
use crate::port::UserPort;

#[derive(Debug, Clone)]
pub struct UserRepository {
    pub db: DatabaseConnection,
}

#[async_trait]
impl UserPort for UserRepository {
    async fn find_user(&self, id: UserId) -> Result<User, AppError> {
        let model = UserEntity::find_by_id(id.0)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find user by id {}: {}", id.0, e);
                AppError::InternalServerError
            })?
            .ok_or(AppError::NotFound)?;
        Ok(User::new(UserId(model.id), model.name))
    }

    async fn find_users(&self) -> Result<Users, AppError> {
        let models = UserEntity::find()
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find users: {}", e);
                AppError::InternalServerError
            })?;
        Ok(models
            .into_iter()
            .map(|m| User::new(UserId(m.id), m.name))
            .collect())
    }
}
