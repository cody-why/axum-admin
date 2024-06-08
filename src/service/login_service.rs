
use super::CONTEXT;
use crate::{error::Result, Error};

const CACHE_KEY_RETRY: &str = "login:login_retry";
const CACHE_KEY_RETRY_TTL: &str = "login:login_retry_ttl";

///is need to wait
pub async fn is_need_wait_login_ex(account: &str) -> Result<u64> {
    if CONTEXT.config.login_fail_retry > 0 {
        let num: Option<u64> = CONTEXT
            .mem_cache_service
            .get_json(&format!("{}{}", CACHE_KEY_RETRY, account))
            .await?;
        let num = num.unwrap_or(0);

        if num >= CONTEXT.config.login_fail_retry {
            let wait_sec: i64 = CONTEXT
                .mem_cache_service
                .ttl(&format!("{}{}", CACHE_KEY_RETRY_TTL, account))
                .await.unwrap_or_default();
            if wait_sec > 0 {
                // let e = error_info!("req_frequently", format!("{}", wait_sec));
                let e = Error::E(format!("操作过于频繁请等待{}秒后重试", wait_sec));
                return Err(e);
            }
        }
        return Ok(num);
    }
    Ok(0)
}

///Add redis retry record
pub async fn add_retry_login_limit_num( account: &str) -> Result<()> {
    if CONTEXT.config.login_fail_retry > 0 {
        let num: Option<u64> = CONTEXT
            .mem_cache_service
            .get_json(&format!("{}{}", CACHE_KEY_RETRY, account))
            .await?;
        let mut num = num.unwrap_or(0);

        num += 1;
        CONTEXT
            .mem_cache_service
            .set_string(
                &format!("{}{}", CACHE_KEY_RETRY, account),
                &num.to_string(),
                60*15,
            )
            .await?;
        CONTEXT
            .mem_cache_service
            .set_string(
                &format!("{}{}", CACHE_KEY_RETRY_TTL, account),
                &num.to_string(),
                CONTEXT.config.login_fail_retry_wait_sec
            )
            .await?;
    }
    Ok(())
}

pub async fn remove_retry_login_limit_num(account: &str) -> Result<()> {
    if CONTEXT.config.login_fail_retry > 0 {
        CONTEXT
            .mem_cache_service
            .remove(
                &format!("{}{}", CACHE_KEY_RETRY, account),
            )
            .await?;
    }
    Ok(())
}