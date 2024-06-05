use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;

use crate::vo::Response;
use crate::AppState;
use crate::model::menu::SysMenu;
use crate::vo::*;
use crate::vo::menu_vo::*;

pub fn router() -> Router<Arc<AppState>>{
    Router::new()
        .route("/menu_list", post(menu_list))
        .route("/menu_save", post(menu_save))
        .route("/menu_delete", post(menu_delete))
        .route("/menu_update", post(menu_update))
}

// 查询菜单
pub async fn menu_list(State(state): State<Arc<AppState>>, Json(item): Json<MenuListReq>) -> impl IntoResponse {
    log::info!("menu_list params: {:?}", &item);
    let rb = &state.batis;

    // 菜单是树形结构不需要分页
    let result = SysMenu::select_all(rb).await;

    let mut menu_list: Vec<MenuListData> = Vec::new();
    match result {
        Ok(sys_menu_list) => {
            for menu in sys_menu_list {
                menu_list.push(MenuListData::from(menu))
            }
            ok_result_page(menu_list, 0)
        }
        Err(err) => {
            err_result_page(menu_list, err)
        }
    }
}

// 添加菜单
pub async fn menu_save(State(state): State<Arc<AppState>>, Json(item): Json<MenuSaveReq>) -> impl IntoResponse {
    log::info!("menu_save params: {:?}", &item);
    let rb = &state.batis;

    let sys_menu = SysMenu::from(item);

    let result = SysMenu::insert(rb, &sys_menu).await;

    Response::result(result)
}

// 更新菜单
pub async fn menu_update(State(state): State<Arc<AppState>>, Json(item): Json<MenuUpdateReq>) -> impl IntoResponse {
    log::info!("menu_update params: {:?}", &item);
    let rb = &state.batis;

    // let sys_menu = SysMenu::from(item);
    
    let result = MenuUpdateReq::update_by_column(rb, &item, "id").await;

    Response::result(result)
}

// 删除菜单信息
pub async fn menu_delete(State(state): State<Arc<AppState>>, Json(item): Json<MenuDeleteReq>) -> impl IntoResponse {
    log::info!("menu_delete params: {:?}", &item);
    let rb = &state.batis;

    //有下级的时候 不能直接删除
    let menus = SysMenu::select_by_column(rb, "parent_id", &item.id).await.unwrap_or_default();

    if !menus.is_empty() {
        return Response::err("有下级菜单,不能直接删除")
    }

    let result = SysMenu::delete_by_column(rb, "id", &item.id).await;

    Response::result(result)
}