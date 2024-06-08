use once_cell::sync::Lazy;
use rbatis::RBatis;

use crate::config::Config;
<<<<<<< HEAD
use crate::config::db::init_db;
=======
use crate::utils::db::init_db;
use crate::utils::cache::cache_service::CacheService;
>>>>>>> dev

pub mod sys_trash_service;
pub mod menu_service;
pub mod user_service;
pub mod role_service;

<<<<<<< HEAD
=======
pub mod login_service;

>>>>>>> dev
/// CONTEXT is all the service struct
pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(||{
    ServiceContext::new()
});

<<<<<<< HEAD
/// erverwhere use the `context!` macro to get a reference to the `ServiceContext` struct.
#[macro_export]
macro_rules! context {
    () => {
        &$crate::service::CONTEXT
    };
}

/// erverwhere use the `pool!` macro to get a reference to the `RBatis` pool.
#[macro_export]
macro_rules! pool {
    () => {
        &$crate::service::CONTEXT.rb
    };
}

=======
>>>>>>> dev
#[derive(Default)]
pub struct ServiceContext {
    pub config: Config,
    pub rb: RBatis,
<<<<<<< HEAD
    // pub cache_service: CacheService,
=======
    pub cache_service: CacheService,
    pub mem_cache_service: CacheService,
>>>>>>> dev
}

impl ServiceContext {

    pub fn new() -> Self {
        let config = Config::new();
<<<<<<< HEAD
        let rb = RBatis::new();

        Self {
            config,
            rb,
=======
        let cache_service = CacheService::new(&config.cache_type).unwrap();
        let mem_cache_service = CacheService::new("mem").unwrap();
        
        Self {
            config,
            rb: RBatis::new(),
            cache_service,
            mem_cache_service,
>>>>>>> dev
        }
    }

    /// must call this method before using any service
    pub async fn init_database(&self) {
        init_db(&self.config, &self.rb).await;
        // CacheService::new(&config).unwrap(),
    }
}