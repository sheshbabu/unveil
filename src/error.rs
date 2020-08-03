use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use std::fmt;

#[derive(Debug)]
pub enum Error {
  SqlError(sqlx::Error),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Internal Server Error")
  }
}

impl ResponseError for Error {
  fn error_response(&self) -> HttpResponse {
    log::error!("Error {:?}", self);
    HttpResponse::build(self.status_code()).body("Internal Server Error")
  }

  fn status_code(&self) -> StatusCode {
    match self {
      Error::SqlError(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

impl From<sqlx::Error> for Error {
  fn from(e: sqlx::Error) -> Self {
    Error::SqlError(e)
  }
}
