use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;
use rbatis::plugin::page::PageRequest;

use crate::vo::Response;
use crate::AppState;
use crate::model::menu::SysMenu;
use crate::model::role::SysRole;
use crate::model::role_menu::{query_menu_by_role, SysRoleMenu};
use crate::model::user_role::SysUserRole;
use crate::vo::*;
use crate::vo::role_vo::*;

pub fn router() -> Router<Arc<AppState>>{
    Router::new()
        .route("/query_role_menu", post(query_role_menu))
        .route("/update_role_menu", post(update_role_menu))
        .route("/role_list", post(role_list))
        .route("/role_save", post(role_save))
        .route("/role_delete", post(role_delete))
        .route("/role_update", post(role_update))
}

// 查询角色列表
pub async fn role_list(State(state): State<Arc<AppState>>, Json(item): Json<RoleListReq>) -> impl IntoResponse {
    log::info!("role_list params: {:?}", &item);
    let rb = &state.batis;

    let role_name = item.role_name.as_deref().unwrap_or_default();
    let status_id = item.status_id.as_deref().unwrap_or_default();

    let page_req = PageRequest::new(item.page_no, item.page_size);
    let result = SysRole::select_page_by_name(rb, &page_req, role_name, status_id).await;

    let mut role_list: Vec<RoleListData> = Vec::new();
    match result {
        Ok(page) => {
            let total = page.total;

            for role in page.records {
                role_list.push(RoleListData::from(role));
            }

            ok_result_page(role_list, total)
        }
        Err(err) => {
            err_result_page(role_list, err)
        }
    }
}

// 添加角色信息
pub async fn role_save(State(state): State<Arc<AppState>>, Json(item): Json<RoleSaveReq>) -> impl IntoResponse {
    log::info!("role_save params: {:?}", &item);
    let rb = &state.batis;

    let sys_role = SysRole::from(item);

    let result = SysRole::insert(rb, &sys_role).await;
    Response::result(result)
}

// 更新角色信息
pub async fn role_update(State(state): State<Arc<AppState>>, Json(item): Json<RoleUpdateReq>) -> impl IntoResponse {
    log::info!("role_update params: {:?}", &item);
    let rb = &state.batis;

    // let sys_role = SysRole::from(item);

    let result = RoleUpdateReq::update_by_column(rb, &item, "id").await;
    Response::result(result)
}

// 删除角色信息
pub async fn role_delete(State(state): State<Arc<AppState>>, Json(item): Json<RoleDeleteReq>) -> impl IntoResponse {
    log::info!("role_delete params: {:?}", &item);
    let rb = &state.batis;

    let ids = item.ids.clone();
    let user_role_list = SysUserRole::select_in_column(rb, "role_id", &ids).await.unwrap_or_default();

    if !user_role_list.is_empty() {
        return Response::err("角色已被使用,不能直接删除");
    }
    let result = SysRole::delete_in_column(rb, "id", &item.ids).await;
    Response::result(result)
}

// 查询角色关联的菜单
pub async fn query_role_menu(State(state): State<Arc<AppState>>, Json(item): Json<QueryRoleMenuReq>) -> impl IntoResponse {
    log::info!("query_role_menu params: {:?}", &item);
    let rb = &state.batis;

    // 查询所有菜单
    let menu_list = SysMenu::select_all(rb).await.unwrap_or_default();

    let mut menu_data_list: Vec<MenuDataList> = Vec::new();
    let mut role_menu_ids: Vec<i32> = Vec::new();

    for y in menu_list {
        let id = y.id.unwrap();
        menu_data_list.push(y.into());
        role_menu_ids.push(id)
    }

    //不是超级管理员的时候,就要查询角色和菜单的关联
    if item.role_id != 1 {
        role_menu_ids.clear();
        let role_menu_list = query_menu_by_role(rb, item.role_id).await.unwrap_or_default();

        for x in role_menu_list {
            let m_id = *x.get("menu_id").unwrap();
            role_menu_ids.push(m_id)
        }
    }
    let result = QueryRoleMenuData {
        role_menus: role_menu_ids,
        menu_list: menu_data_list,
    };
    Response::ok(result)
}

// 更新角色关联的菜单
pub async fn update_role_menu(State(state): State<Arc<AppState>>, Json(item): Json<UpdateRoleMenuReq>) -> impl IntoResponse {
    log::info!("update_role_menu params: {:?}", &item);
    let role_id = item.role_id;

    let rb = &state.batis;

    let role_menu_result = SysRoleMenu::delete_by_column(rb, "role_id", &role_id).await;

    match role_menu_result {
        Ok(_) => {
            let mut menu_role: Vec<SysRoleMenu> = Vec::new();

            for id in &item.menu_ids {
                let menu_id = *id;
                menu_role.push(SysRoleMenu::new(role_id, menu_id))
            }

            let result = SysRoleMenu::insert_batch(rb, &menu_role, item.menu_ids.len() as u64).await;
            Response::result(result)
        }
        Err(err) => {
            Response::err(err)
        }
    }
}
