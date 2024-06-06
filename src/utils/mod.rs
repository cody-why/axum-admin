use std::time::SystemTime;

pub mod jwt_util;
pub mod redis_util;
pub mod password;

pub fn get_timestamp() -> u64 {
    let now = SystemTime::now();
    now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()

}