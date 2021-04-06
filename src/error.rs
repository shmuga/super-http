use actix_web::HttpResponse;
use actix_web::ResponseError;
use tantivy::{TantivyError, query::QueryParserError};
use std::io::Error as IoError;
use std::convert::From;


#[derive(Fail, Debug)]
pub enum Error {
    // 401
    #[fail(display = "Unauthorized")]
    Unauthorized,

    // 403
    #[fail(display = "Forbidden")]
    Forbidden,

    // 404
    #[fail(display = "Not Found")]
    NotFound,

    // 422
    #[fail(display = "Unprocessable Entity")]
    UnprocessableEntity,

    // 500
    #[fail(display = "Internal Server Error")]
    InternalServerError,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json("Internal server error")
    }
}


impl From<TantivyError> for Error {
    fn from(_e: TantivyError) -> Self {
        Error::UnprocessableEntity
    }
}

impl From<IoError> for Error {
    fn from(_e: IoError) -> Self {
        Error::InternalServerError
    }
}

impl From <QueryParserError> for Error {
    fn from(_e: QueryParserError) -> Self {
        Error::InternalServerError
    }
}

