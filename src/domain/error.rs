use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("An internal error occurred. Please try again later.")]
    InternalServerError,
    #[error("Not Found")]
    NotFound,
}

impl actix_web::ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::InternalServerError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => actix_web::http::StatusCode::NOT_FOUND,
        }
    }
}
