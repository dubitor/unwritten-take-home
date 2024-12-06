use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::derive::{Display, Error, From};
use log::error;
use polars::error::PolarsError;
use tokio::task::JoinError;

#[derive(Debug, Display, Error, From)]
pub enum UnwrittenError {
    PostgressError(tokio_postgres::Error),
    DbConnectionPoolError(PoolError),
    DataNotFound,
    DataFrameError(PolarsError),
    TokioJoinError(JoinError),
}
impl ResponseError for UnwrittenError {
    // Log the internal error then map it to a response.
    // We would be more selective about what goes in the response body for a real customer-facing
    // application.
    fn error_response(&self) -> HttpResponse {
        error!("{}", self);
        match *self {
            UnwrittenError::DataNotFound => HttpResponse::NotFound().finish(),
            UnwrittenError::PostgressError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            UnwrittenError::DataFrameError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            UnwrittenError::DbConnectionPoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            UnwrittenError::TokioJoinError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
        }
    }
}
