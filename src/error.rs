use std::fmt::Debug;
use rbatis::rbdc;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    E(String),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Db(#[from] rbdc::Error),
    #[error("{0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("{0}: {1}")]
    Code(String, String),
}


impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::E(s.to_string())
    }
}

#[macro_export]
macro_rules! error_info {
    ($code: expr) => {
        $crate::service::CONTEXT.config.get_error($code)
    };
    ($code: expr, $arg: expr) => {
        $crate::service::CONTEXT.config.get_error_arg($code, $arg)
    };
}

