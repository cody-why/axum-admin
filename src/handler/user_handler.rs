use axum::Json;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::{get, post};
use log::info;
use crate::service::user_service;
use crate::middleware::context::UserContext;
use crate::vo::*;
use crate::vo::user_vo::*;

pub fn router() -> Router{
    Router::new()
        .route("/login", post(login))
        .route("/query_user_role", post(query_user_role))
        .route("/update_user_role", post(update_user_role))
        .route("/query_user_menu", get(query_user_menu))
        .route("/user_list", post(user_list))
        .route("/user_save", post(user_save))
        .route("/user_delete", post(user_delete))
        .route("/user_update", post(user_update))
        .route("/update_user_password", post(update_user_password))
}

// 后台用户登录
pub async fn login(Json(item): Json<UserLoginReq>) -> impl IntoResponse {
    info!("user login params: {:?}", &item);
    let result = user_service::login(item).await;
    Response::result(result)
}

pub async fn query_user_role(Json(item): Json<QueryUserRoleReq>) -> impl IntoResponse {
    info!("query_user_role params: {:?}", item);
    
    let result = user_service::query_user_role(item).await;
    Response::result(result)

}

pub async fn update_user_role(Json(item): Json<UpdateUserRoleReq>) -> impl IntoResponse {
    info!("update_user_role params: {:?}", item);
    let result = user_service::update_user_role(item).await;
    Response::result(result)
}

pub async fn query_user_menu(content: UserContext) -> impl IntoResponse {
    info!("query user menu params {:?}", content);

    let result = user_service::query_user_menu(content).await;
    Response::result(result)
    
}

// 查询用户列表
pub async fn user_list(Json(item): Json<UserListReq>) -> impl IntoResponse {
    info!("query user_list params: {:?}", &item);
    let result = user_service::user_list(item).await;
    let total = result.as_ref().map_or(0, |data| data.total);
    // 转换成前端需要的格式
    let result = result.map(|data| data.records);
    Response::result_page(result, total)
}

// 添加用户信息
pub async fn user_save(Json(item): Json<UserSaveReq>) -> impl IntoResponse {
    info!("user_save params: {:?}", &item);

    let result = user_service::user_save(item).await;
    Response::result(result)
}

// 更新用户信息
pub async fn user_update(Json(item): Json<UserUpdateReq>) -> impl IntoResponse {
    info!("user_update params: {:?}", &item);

    let result = user_service::user_update(item).await;
    Response::result(result)
}

// 删除用户信息
pub async fn user_delete(Json(item): Json<UserDeleteReq>) -> impl IntoResponse {
    info!("user_delete params: {:?}", &item);
    let result = user_service::user_delete(item).await;
    Response::result(result)
}

// 更新用户密码
pub async fn update_user_password(Json(item): Json<UpdateUserPwdReq>) -> impl IntoResponse {
    info!("update_user_pwd params: {:?}", &item);

    let result = user_service::update_user_password(item).await;
    Response::result(result)
}