use std::time::SystemTime;

pub mod jwt_util;
pub mod redis;
pub mod password;
pub mod cache;
pub mod db;
mod macros;

pub fn get_timestamp() -> u64 {
    let now = SystemTime::now();
    now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()

}