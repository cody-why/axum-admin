use rbatis::rbdc;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    E(String),
    #[error("其他")]
    Internal(String),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Db(#[from] rbdc::Error),
    #[error("{0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("{0}: {1}")]
    Code(String, String),
}

impl Error {
    pub fn err<T>(s: impl Into<String>) -> Result<T> {
        Err(Error::E(s.into()))
    }
}

impl Error {
    pub fn msg(&self) -> String {
        match self {
            Error::E(s) => s.clone(),
            Error::Code(code, message) => format!("{}: {}", code, message),
            _ => "Internal Server Error".to_string(),
        }
    }

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

