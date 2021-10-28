use reqwest::Error as ReqwestError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("http_client error")]
    HttpError(#[from] ReqwestError),
    #[error("unknown data store error")]
    Unknown,
}
