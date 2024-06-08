
use std::time::Duration;

use axum::http::{Request, Response};
use axum::{middleware, Router};
use axum::response::IntoResponse;
use axum::routing::get;
use tower_http::{cors::{Any, CorsLayer}, services::{ServeDir, ServeFile}, trace::TraceLayer};

use crate::{middleware::auth::auth, pool};
use super::{menu_handler, role_handler, user_handler};

pub fn app() -> Router {
    // let app_state = Arc::new(AppState{batis: CONTEXT.rb.clone() });
    // let origins = [
    //     "http://localhost:3000".parse().unwrap(),
    // ];
    let trace_layer = TraceLayer::new_for_http()
        .on_request(|request: &Request<_>, _span: &_| {
            log::debug!("on_request: {} {}", request.method(), request.uri().path());
        })
        .on_response(|response: &Response<_>, latency: Duration, _span: &_| {
            log::debug!("on_response: {:?}ms, {:?}", latency.as_millis(), response.status());
        });
    let cors_layer = CorsLayer::new().allow_methods(Any).allow_origin(Any).allow_headers(Any);
    
    Router::new()

        .nest("/api", Router::new()
        .merge(user_handler::router())
        .merge(role_handler::router())
        .merge(menu_handler::router())
        // .with_state(app_state)
    )
        .route_layer(middleware::from_fn(auth))
        .route("/status", get(db_status))
        .layer(trace_layer)
        .layer(cors_layer)
        .merge(static_file())
}

async fn db_status() -> impl IntoResponse {
    let state = pool!().get_pool().expect("pool not init!").state().await;
    state.to_string()

}

pub fn static_file() -> Router {
    Router::new()
    .nest_service("/", ServeDir::new("dist/")
       .not_found_service(ServeFile::new("dist/index.html")))
}