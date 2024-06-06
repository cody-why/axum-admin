use std::time::Duration;
use rbatis::RBatis;
use log::info;
use crate::config::Config;

/// init database pool
pub async fn init_db(config: &Config, rb: &RBatis){
    info!("rbatis pool init ({:?})...", config.db.url.split('@').nth(1));
    // let mut rb = RBatis::new();
    rb.link(rbdc_mysql::Driver{}, config.db.url.as_str())
        .await
        .expect("[abs_admin] rbatis pool init fail!");
    // rb.intercepts.push(Arc::new(SysTrashService::new()));
    let pool = rb.get_pool().unwrap();
    //max connections
    pool.set_max_open_conns(config.db.max_connections as u64).await;
    pool.set_max_idle_conns(config.db.min_connections as u64).await;
    //max timeout
    pool.set_timeout(Some(Duration::from_secs(config.db.connect_timeout as u64))).await;
    let state = rb.get_pool().expect("pool not init!").state().await;
    info!("Rbatis pool init success! pool state = {}", state);

}
