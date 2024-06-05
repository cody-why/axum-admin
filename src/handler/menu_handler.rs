
use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::post;
use tracing::info;

use crate::service::menu_service;
use crate::vo::Response;
use crate::vo::menu_vo::*;

pub fn router() -> Router {
    Router::new()
        .route("/menu_list", post(menu_list))
        .route("/menu_save", post(menu_save))
        .route("/menu_delete", post(menu_delete))
        .route("/menu_update", post(menu_update))
}

// 查询菜单
pub async fn menu_list(Json(item): Json<MenuListReq>) -> impl IntoResponse {
    info!("menu_list params: {:?}", &item);

    let result = menu_service::menu_list().await;
    Response::result_page(result, 0)

}

// 添加菜单
pub async fn menu_save(Json(item): Json<MenuSaveReq>) -> impl IntoResponse {
    info!("menu_save params: {:?}", &item);

    let result = menu_service::menu_save(item).await;
    Response::result(result)
}

// 更新菜单
pub async fn menu_update(Json(item): Json<MenuUpdateReq>) -> impl IntoResponse {
    info!("menu_update params: {:?}", &item);

    let result = menu_service::menu_update(item).await;
    Response::result(result)
}

// 删除菜单信息
pub async fn menu_delete(Json(item): Json<MenuDeleteReq>) -> impl IntoResponse {
    info!("menu_delete params: {:?}", &item);

    let result = menu_service::menu_delete(item).await;
    Response::result(result)
}