#[macro_use]
extern crate rbatis;
pub use error::*;

pub mod config;
pub mod model;
pub mod vo;
pub mod handler;
pub mod utils;
pub mod middleware;
pub mod service;
pub mod error;

use handler::root::*;
use rbatis::RBatis;
use log::info;
use crate::service::CONTEXT;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

pub struct AppState {
    pub batis: RBatis,
}

#[tokio::main]
async fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    CONTEXT.init_database().await;

    
    let app = app();

    let addr = CONTEXT.config.addr.as_str();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();

}


