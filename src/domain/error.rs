use actix_web::{http::StatusCode, HttpResponse};
use failure::Fail;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "An internal error occurred. Please try again later.")]
    InternalServerError,
    #[fail(display = "Not Found")]
    NotFound,
}
