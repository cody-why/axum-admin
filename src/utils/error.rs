use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    E(String),
    #[error("{0}")]
    Io(std::io::Error),
    #[error("{0}")]
    JwtToken(jsonwebtoken::errors::Error),
    #[error("{0}")]
    Code(String, String),
}