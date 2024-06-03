#[macro_use]
extern crate rbatis;

pub mod model;
pub mod vo;
pub mod handler;
pub mod utils;
pub mod middleware;

use handler::root::*;
use std::sync::Arc;
use rbatis::RBatis;
use crate::model::db::init_db;

pub struct AppState {
    pub batis: RBatis,
}

#[tokio::main]
async fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    let rb = init_db().await;
    let app_state = Arc::new(AppState{batis: rb.clone() });
    let app = app(app_state);

    // axum 0.7.x
    let addr = "127.0.0.1:8000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    log::info!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();

    
}


