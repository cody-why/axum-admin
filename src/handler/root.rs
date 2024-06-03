use std::sync::Arc;

use axum::{middleware, Router};
use tower_http::{cors::{Any, CorsLayer}, services::{ServeDir, ServeFile}, trace::TraceLayer};

use crate::{AppState, middleware::auth::auth};

use super::{menu_handler, role_handler, user_handler};

pub fn app(app_state: Arc<AppState>) -> Router {
    // let origins = [
    //     "http://localhost:3000".parse().unwrap(),
    // ];
    let trace_layer = TraceLayer::new_for_http();
    let cors_layer = CorsLayer::new().allow_methods(Any).allow_origin(Any).allow_headers(Any);
    
    Router::new()
        .nest("/api", Router::new()
        .merge(user_handler::router())
        .merge(role_handler::router())
        .merge(menu_handler::router())
        .with_state(app_state)
    )
    .route_layer(middleware::from_fn(auth))
    .layer(trace_layer)
    .layer(cors_layer)
    .merge(app2())
}

pub fn app2() -> Router {
    Router::new()
    .nest_service("/", ServeDir::new("dist/")
       .not_found_service(ServeFile::new("dist/index.html")))
}