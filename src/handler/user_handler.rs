use std::collections::HashSet;
use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::{get, post};
use rbatis::plugin::page::PageRequest;
use rbatis::RBatis;
use rbatis::rbdc::datetime::DateTime;
use rbs::to_value;

use crate::AppState;
use crate::middleware::context::UserContext;
use crate::model::menu::{SysMenu, SysMenuUrl};
use crate::model::role::SysRole;
use crate::model::user::SysUser;
use crate::model::user_role::SysUserRole;
use crate::utils::jwt_util::JWTToken;
use crate::utils::password::Password;
use crate::vo::{BaseResponse, err_result_msg, err_result_page, handle_result, ok_result_data, ok_result_msg, ok_result_page};
use crate::vo::user_vo::*;

pub fn router() -> Router<Arc<AppState>>{
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
pub async fn login(State(state): State<Arc<AppState>>, Json(item): Json<UserLoginReq>) -> impl IntoResponse {
    log::info!("user login params: {:?}", &item);
    let rb = &state.batis;

    let user_result = SysUser::select_by_mobile(rb, &item.mobile).await;
    // log::info!("select_by_mobile: {:?}", user_result);

    let user= match user_result {
        Ok(Some(user)) => {
            user
        }
        Ok(None) => {
            return err_result_msg("用户不存在".to_string())
        }
        Err(e) => {
            log::info!("select_by_mobile err: {:?}", e);
            return err_result_msg("查询用户异常".to_string())
        }
    };

    let id = user.id.unwrap();
    let username = user.user_name;

    if !Password::verify(&item.password, &user.password) {
        return err_result_msg("密码不正确".to_string())
    }

    let btn_menu = query_btn_menu(id, rb.clone()).await;

    if btn_menu.is_empty() {
        return err_result_msg("用户没有分配角色或者菜单,不能登录".to_string())
    }
    JWTToken::new(id, &username, btn_menu).create_token("123")
        .map(ok_result_data)
        .map_err(|e| {
            err_result_msg(e.to_string())
        }).unwrap()


}

async fn query_btn_menu(id: u64, rb: RBatis) -> Vec<String> {
    let user_role = SysUserRole::is_admin(&rb, id).await;

    if user_role.unwrap().len() == 1 {
        log::info!("admin login: {:?}",id);
        let data = SysMenu::select_all(&rb).await.unwrap_or_default();
        data.into_iter().filter_map(|x| x.api_url.filter(|x| !x.is_empty())).collect()

    } else {
        log::info!("ordinary login: {:?}",id);

        // distinct--返回不重复的数据
        let sql = "select distinct m.api_url from sys_user_role ur
                left join sys_role r on ur.role_id = r.id
                left join sys_role_menu rm on r.id = rm.role_id
                left join sys_menu m on rm.menu_id = m.id where ur.user_id = ?";
        // let btn_menu_map: Vec<HashMap<String, String>> = rb.query_decode(sql, vec![to_value!(id)]).await.unwrap();
        let btn_menu_vec: Vec<SysMenuUrl> = rb.query_decode(sql, vec![to_value!(id)]).await.unwrap_or_default();
        log::info!("btn_menu_vec: {:?}",btn_menu_vec);
        btn_menu_vec.into_iter().filter_map(|x| x.api_url.filter(|x|!x.is_empty())).collect()

    }
}

pub async fn query_user_role(State(state): State<Arc<AppState>>, Json(item): Json<QueryUserRoleReq>) -> impl IntoResponse {
    log::info!("query_user_role params: {:?}", item);
    let rb = &state.batis;

    let user_role = SysUserRole::select_by_column(rb, "user_id", item.user_id).await;
    let mut user_role_ids: Vec<i32> = Vec::new();

    for x in user_role.unwrap() {
        user_role_ids.push(x.role_id);
    }

    let sys_role = SysRole::select_all(rb).await;

    let mut sys_role_list: Vec<UserRoleList> = Vec::new();

    for x in sys_role.unwrap() {
        sys_role_list.push(x.into());
    }

    Json(ok_result_data(QueryUserRoleData {
        sys_role_list,
        user_role_ids,
    }))
}

pub async fn update_user_role(State(state): State<Arc<AppState>>, Json(item): Json<UpdateUserRoleReq>) -> impl IntoResponse {
    log::info!("update_user_role params: {:?}", item);
    let rb = &state.batis;

    let user_id = item.user_id;
    let role_ids = &item.role_ids;
    let len = item.role_ids.len();

    if user_id == 1 {
        return err_result_msg("不能修改超级管理员的角色".to_string())
    }

    let sys_result = SysUserRole::delete_by_column(rb, "user_id", user_id).await;

    if sys_result.is_err() {
        return err_result_msg("更新用户角色异常".to_string())
    }

    let mut sys_role_user_list: Vec<SysUserRole> = Vec::new();
    for role_id in role_ids {
        let r_id = *role_id;
        sys_role_user_list.push(SysUserRole {
            id: None,
            create_time: Some(DateTime::now()),
            update_time: Some(DateTime::now()),
            status_id: 1,
            sort: 1,
            role_id: r_id,
            user_id,
        })
    }

    let result = SysUserRole::insert_batch(rb, &sys_role_user_list, len as u64).await;

    handle_result(result)
}

pub async fn query_user_menu(State(state): State<Arc<AppState>>, content: UserContext) -> impl IntoResponse {
    log::info!("query user menu params {:?}", content);

    let rb = &state.batis;
    let result = SysUser::select_by_id(rb, content.id).await;

    match result {
        Ok(sys_user) => {
            match sys_user {
                // 用户不存在的情况
                None => {
                    Json(BaseResponse {
                        msg: "用户不存在".to_string(),
                        code: 1,
                        data: None,
                    })
                }
                Some(user) => {
                    //role_id为1是超级管理员--判断是不是超级管理员
                    let sql = "select count(id) from sys_user_role where role_id = 1 and user_id = ?";
                    let count = rb.query_decode::<i32>(sql, vec![to_value!(user.id)]).await.unwrap_or_default();

                    let sys_menu_list: Vec<SysMenu> =
                    if count > 0 {
                        SysMenu::select_all(rb).await.unwrap_or_default()
                    } else {
                        let sql = "select m.* from sys_user_role ur
                                left join sys_role r on ur.role_id = r.id
                                left join sys_role_menu rm on r.id = rm.role_id
                                left join sys_menu m on rm.menu_id = m.id where ur.user_id = ?";
                        rb.query_decode(sql, vec![to_value!(user.id)]).await.unwrap()
                    };
                    log::info!("sys_menu_list: {:?}",sys_menu_list.len());
                    let mut btn_menu: Vec<String> = Vec::new();
                    let mut sys_menu_ids: HashSet<i32> = HashSet::new();

                    for x in sys_menu_list {
                        if x.menu_type != 3 {
                            sys_menu_ids.insert(x.id.unwrap_or_default());
                            sys_menu_ids.insert(x.parent_id);
                        }
                        let api_url = x.api_url.unwrap_or_default();
                        if !api_url.is_empty() {
                            btn_menu.push(api_url);
                        }
                    }

                    let mut menu_ids = sys_menu_ids.into_iter().filter(|x| *x!= 0).collect::<Vec<i32>>();
                    menu_ids.sort();
                    let menu_result = SysMenu::select_by_ids(rb, &menu_ids).await.unwrap();

                    let sys_menu: Vec<MenuUserList> = menu_result.into_iter().map(|x| {x.into()}).collect();

                    let resp = BaseResponse {
                        msg: "successful".to_string(),
                        code: 0,
                        data: Some(QueryUserMenuData {
                            sys_menu,
                            btn_menu,
                            avatar: "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png".to_string(),
                            name: user.user_name,
                        }),
                    };
                    Json(resp)
                }
            }
        }
        // 查询用户数据库异常
        Err(err) => {
            Json(BaseResponse {
                msg: err.to_string(),
                code: 1,
                data: None,
            })
        }
    }
}

// 查询用户列表
pub async fn user_list(State(state): State<Arc<AppState>>, Json(item): Json<UserListReq>) -> impl IntoResponse {
    log::info!("query user_list params: {:?}", &item);
    let rb = &state.batis;

    let mobile = item.mobile.as_deref().unwrap_or_default();
    let status_id = item.status_id.as_deref().unwrap_or_default();
    let page_req = &PageRequest::new(item.page_no, item.page_size);
    let result = SysUser::select_page_by_name(rb, page_req, mobile, status_id).await;

    let list_data: Vec<UserListData> = Vec::new();
    match result {
        Ok(page) => {
            let total = page.total;
            let list_data: Vec<UserListData> =  page.records.into_iter().map(|user| {
                user.into()
            }).collect();

            Json(ok_result_page(list_data, total))
        }
        Err(err) => {
            Json(err_result_page(list_data, err.to_string()))
        }
    }
}

// 添加用户信息
pub async fn user_save(State(state): State<Arc<AppState>>, Json(item): Json<UserSaveReq>) -> impl IntoResponse {
    log::info!("user_save params: {:?}", &item);

    let rb = &state.batis;
    let mut sys_user = SysUser::from(item);
    sys_user.password = Password::md5_and_hash(&sys_user.password);

    let result = SysUser::insert(rb, &sys_user).await;

    handle_result(result)
}

// 更新用户信息
pub async fn user_update(State(state): State<Arc<AppState>>, Json(item): Json<UserUpdateReq>) -> impl IntoResponse {
    log::info!("user_update params: {:?}", &item);

    let rb = &state.batis;
    let result = SysUser::select_by_id(rb, item.id).await.unwrap();

    match result {
        None => {
            err_result_msg("用户不存在".to_string())
        }
        Some(_user) => {
            let result = UserUpdateReq::update_by_column(rb, &item, "id").await;

            handle_result(result)
        }
    }
}

// 删除用户信息
pub async fn user_delete(State(state): State<Arc<AppState>>, Json(item): Json<UserDeleteReq>) -> impl IntoResponse {
    log::info!("user_delete params: {:?}", &item);
    let rb = &state.batis;

    let ids = item.ids.clone();
    for id in ids {
        if id != 1 {//id为1的用户为系统预留用户,不能删除
            let _ = SysUser::delete_by_column(rb, "id", &id).await;
        }
    }

    Json(ok_result_msg("删除用户信息成功".to_string()))
}

// 更新用户密码
pub async fn update_user_password(State(state): State<Arc<AppState>>, Json(item): Json<UpdateUserPwdReq>) -> impl IntoResponse {
    log::info!("update_user_pwd params: {:?}", &item);

    let rb = &state.batis;

    let sys_user_result = SysUser::select_by_id(rb, item.id).await;

    match sys_user_result {
        Ok(user_result) => {
            match user_result {
                None => {
                    err_result_msg("用户不存在".to_string())
                }
                Some(user) => {
                    if Password::verify(&item.pwd, &user.password) {
                        let id = user.id.unwrap();
                        let password = Password::hash(&item.re_pwd);
                        let result = SysUser::update_password(rb, &user, id, &password).await;

                        handle_result(result)
                    } else {
                        err_result_msg("旧密码不正确".to_string())
                    }
                }
            }
        }
        Err(err) => {
            err_result_msg(err.to_string())
        }
    }
}