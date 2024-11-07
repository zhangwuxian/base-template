use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommonError {
    #[error("{0}")]
    CommonError(String),
    #[error("io error")]
    IOError(#[from] io::Error),
}
