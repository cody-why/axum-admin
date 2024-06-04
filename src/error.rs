use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    E(String),
    #[error("{0}")]
    Io(std::io::Error),

    #[error("{0}: {1}")]
    Code(String, String),
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

