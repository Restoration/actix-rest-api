use std::sync::Arc;
use crate::interactor::UserUseCase;

pub struct Container {
    pub user_use_case: Arc<dyn UserUseCase>,
}
