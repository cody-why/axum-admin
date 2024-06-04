#[macro_use]
extern crate rbatis;

pub mod config;
pub mod model;
pub mod vo;
pub mod handler;
pub mod utils;
pub mod middleware;
pub mod service;
pub mod error;

use handler::root::*;
use std::sync::Arc;
use rbatis::RBatis;
use crate::service::CONTEXT;

pub struct AppState {
    pub batis: RBatis,
}

#[tokio::main]
async fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    CONTEXT.init_service().await;

    let app_state = Arc::new(AppState{batis: CONTEXT.rb.clone() });
    let app = app(app_state);

    let addr = CONTEXT.config.addr.as_str();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    log::info!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();

}


