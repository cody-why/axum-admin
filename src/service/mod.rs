use once_cell::sync::Lazy;
use rbatis::RBatis;

use crate::config::Config;
use crate::utils::db::init_db;
use crate::utils::cache::cache_service::CacheService;

pub mod sys_trash_service;
pub mod menu_service;
pub mod user_service;
pub mod role_service;

pub mod login_service;

/// CONTEXT is all the service struct
pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(||{
    ServiceContext::new()
});

#[derive(Default)]
pub struct ServiceContext {
    pub config: Config,
    pub rb: RBatis,
    pub cache_service: CacheService,
    pub mem_cache_service: CacheService,
}

impl ServiceContext {

    pub fn new() -> Self {
        let config = Config::new();
        let cache_service = CacheService::new(&config.cache_type).unwrap();
        let mem_cache_service = CacheService::new("mem").unwrap();
        
        Self {
            config,
            rb: RBatis::new(),
            cache_service,
            mem_cache_service,
        }
    }

    /// must call this method before using any service
    pub async fn init_database(&self) {
        init_db(&self.config, &self.rb).await;
        // CacheService::new(&config).unwrap(),
    }
}