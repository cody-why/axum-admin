use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::post;
use log::info;

use crate::service::role_service;
use crate::vo::*;
use crate::vo::role_vo::*;

pub fn router() -> Router
{
    Router::new()
        .route("/query_role_menu", post(query_role_menu))
        .route("/update_role_menu", post(update_role_menu))
        .route("/role_list", post(role_list))
        .route("/role_save", post(role_save))
        .route("/role_delete", post(role_delete))
        .route("/role_update", post(role_update))
}

// 查询角色列表
pub async fn role_list(Json(item): Json<RoleListReq>) -> impl IntoResponse {
    info!("role_list params: {:?}", &item);
    
    let result = role_service::role_list(item).await;
    let total = result.as_ref().map_or(0, |data| data.total);
     // 转换成前端需要的格式
    let result = result.map(|data| data.records);
    Response::result_page(result, total)
}

// 添加角色信息
pub async fn role_save(Json(item): Json<RoleSaveReq>) -> impl IntoResponse {
    info!("role_save params: {:?}", &item);
    let result = role_service::role_save(item).await;

    Response::result(result)
    
}

// 更新角色信息
pub async fn role_update(Json(item): Json<RoleUpdateReq>) -> impl IntoResponse {
    info!("role_update params: {:?}", &item);
    
    let result = role_service::role_update(item).await;
    Response::result(result)
}
    
// 删除角色信息
pub async fn role_delete(Json(item): Json<RoleDeleteReq>) -> impl IntoResponse {
    info!("role_delete params: {:?}", &item);
    
    let result = role_service::role_delete(item).await;
    Response::result(result)
}

// 查询角色关联的菜单
pub async fn query_role_menu(Json(item): Json<QueryRoleMenuReq>) -> impl IntoResponse {
    info!("query_role_menu params: {:?}", &item);
    let result = role_service::query_role_menu(item).await;
    Response::result(result)
}

// 更新角色关联的菜单
pub async fn update_role_menu(Json(item): Json<UpdateRoleMenuReq>) -> impl IntoResponse {
    info!("update_role_menu params: {:?}", &item);
    let result = role_service::update_role_menu(item).await;
    Response::result(result)
}
