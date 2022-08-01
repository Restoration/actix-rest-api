use crate::repository::user_repository::UserRepository;

#[derive(Debug, Clone, Copy)]
pub struct Container {
    user_port: UserRepository,
}
