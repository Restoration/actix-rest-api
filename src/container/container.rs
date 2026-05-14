use std::sync::Arc;
use crate::interactor::user_interactor::UserUseCase;

pub struct Container {
    pub user_use_case: Arc<dyn UserUseCase>,
}
