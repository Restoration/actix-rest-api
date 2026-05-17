use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum AppError {
    #[error("An internal error occurred. Please try again later.")]
    InternalServerError,
    #[error("Not Found")]
    NotFound,
}
