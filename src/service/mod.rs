use once_cell::sync::Lazy;
use rbatis::RBatis;

use crate::config::Config;
use crate::config::db::init_db;

pub mod sys_trash_service;
pub mod menu_service;
pub mod user_service;
pub mod role_service;

/// CONTEXT is all the service struct
pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(||{
    ServiceContext::new()
});


#[macro_export]
macro_rules! context {
    () => {
        &$crate::service::CONTEXT
    };
}

#[macro_export]
macro_rules! pool {
    () => {
        &$crate::service::CONTEXT.rb
    };
}

#[derive(Default)]
pub struct ServiceContext {
    pub config: Config,
    pub rb: RBatis,
    // pub cache_service: CacheService,
}

impl ServiceContext {

    pub fn new() -> Self {
        let config = Config::new();
        let rb = RBatis::new();

        Self {
            config,
            rb,
        }
    }

    /// must call this method before using any service
    pub async fn init_database(&self) {
        init_db(&self.config, &self.rb).await;
        // CacheService::new(&config).unwrap(),
    }
}