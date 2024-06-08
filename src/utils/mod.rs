use std::time::SystemTime;

pub mod jwt_util;
<<<<<<< HEAD
pub mod redis_util;
pub mod password;
=======
pub mod redis;
pub mod password;
pub mod cache;
pub mod db;
mod macros;
>>>>>>> dev

pub fn get_timestamp() -> u64 {
    let now = SystemTime::now();
    now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()

}