use std::collections::HashSet;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rbatis::plugin::page::PageRequest;
use rbatis::Page;
use rbatis::rbdc::datetime::DateTime;
use rbs::to_value;
use log::info;
use crate::service::login_service;
use crate::{pool, Error};
use crate::middleware::context::UserContext;
use crate::model::menu::{SysMenu, SysMenuUrl};
use crate::model::role::SysRole;
use crate::model::user::SysUser;
use crate::model::user_role::SysUserRole;
use crate::utils::jwt_util::JWTToken;
use crate::utils::password::Password;
use crate::vo::user_vo::*;
use crate::Result;


// 后台用户登录
pub async fn login(item: UserLoginReq) -> Result<String> {
    let try_num = login_service::is_need_wait_login_ex(&item.mobile).await?;

    let rb = pool!();

    let user_result = SysUser::select_by_mobile(rb, &item.mobile).await;
    // info!("select_by_mobile: {:?}", user_result);

    let user= match user_result {
        Ok(Some(user)) => {
            user
        }
        Ok(None) => {
            return Error::err("用户不存在")
        }
        Err(e) => {
            info!("select_by_mobile err: {:?}", e);
            return Error::err("查询用户异常")
        }
    };
    if !Password::verify(&item.password, &user.password) {
        login_service::add_retry_login_limit_num(&item.mobile).await?;
        
        return Error::err("密码不正确")
    }
    if try_num > 0 {
        login_service::remove_retry_login_limit_num(&item.mobile).await?;
    }
    if user.status_id!= 1 {
        return Error::err("用户已被禁用")
    }
    
    let id = user.id.unwrap();
    let username = user.user_name;

    let btn_menu = query_btn_menu(id).await;
    // info!("btn_menu: {:?}", btn_menu);
    if btn_menu.is_empty() {
        return Error::err("用户没有分配角色或者菜单,不能登录")
    }

    let token = JWTToken::new(id, &username, btn_menu).create_token()?;
    Ok(token)
}

async fn query_btn_menu(id: u64) -> Vec<String> {
    let rb = pool!();
    let user_role = SysUserRole::is_admin(rb, id).await;
    if user_role.is_err() {
        return vec![]
    }
    
    if user_role.unwrap().len() == 1 {
        info!("admin login: {:?}",id);
        let data = SysMenu::select_all(rb).await.unwrap_or_default();
        // info!("btn_menu_vec: {:?}",data);
        data.par_iter().filter_map(|x|{
            x.api_url.as_ref().filter(|u|!u.is_empty())
        }).cloned().collect()
       

    } else {
        info!("ordinary login: {:?}",id);

        // distinct--返回不重复的数据
        let sql = "select distinct m.api_url from sys_user_role ur 
                left join sys_role r on ur.role_id = r.id 
                left join sys_role_menu rm on r.id = rm.role_id 
                left join sys_menu m on rm.menu_id = m.id where ur.user_id = ?";
        let data: Vec<SysMenuUrl> = rb.query_decode(sql, vec![to_value!(id)]).await.unwrap_or_default();
        // info!("btn_menu_vec: {:?}",data);
        data.par_iter().filter_map(|x|{
            x.api_url.as_ref().filter(|u|!u.is_empty())
        }).cloned().collect()

    }
   
}

pub async fn query_user_role(item: QueryUserRoleReq) -> Result<QueryUserRoleData> {
    let rb = pool!();

    let user_role = SysUserRole::select_by_column(rb, "user_id", item.user_id).await?;
    let user_role_ids: Vec<i32> = user_role.par_iter().map(|x| x.role_id).collect();

    let sys_role = SysRole::select_all(rb).await?;

    let sys_role_list: Vec<UserRoleList> = sys_role.into_par_iter().map(|x| x.into()).collect();

    let result =QueryUserRoleData {
        sys_role_list,
        user_role_ids,
    };
    Ok(result)

}

pub async fn update_user_role(item: UpdateUserRoleReq) -> Result<u64> {
    let rb = pool!();

    let user_id = item.user_id;
    let role_ids = &item.role_ids;
    let len = item.role_ids.len();

    if user_id == 1 {
        return Error::err("不能修改超级管理员的角色")
    }

    let _ = SysUserRole::delete_by_column(rb, "user_id", user_id).await?;

    let time = Some(DateTime::now());
    let mut sys_role_user_list: Vec<SysUserRole> = Vec::new();
    for role_id in role_ids {
        let r_id = *role_id;
        sys_role_user_list.push(SysUserRole {
            id: None,
            create_time: time.clone(),
            update_time: time.clone(),
            status_id: 1,
            sort: 1,
            role_id: r_id,
            user_id,
        })
    }

    let result = SysUserRole::insert_batch(rb, &sys_role_user_list, len as u64).await?;

    Ok(result.rows_affected)
}

pub async fn query_user_menu(content: UserContext) -> Result<QueryUserMenuData> {

    let rb = pool!();
    let result = SysUser::select_by_id(rb, content.id).await?;

 
    match result {
        None => {
            Error::err("用户不存在")
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
            info!("sys_menu_list: {:?}",sys_menu_list.len());
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
            let resp = QueryUserMenuData {
                sys_menu,
                btn_menu,
                avatar: "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png".to_string(),
                name: user.user_name,
            };
            Ok(resp)
        }
    }
       
}


// 查询用户列表
pub async fn user_list(item: UserListReq) -> Result<Page<UserListData>> {
    let rb = pool!();

    let mobile = item.mobile.as_deref().unwrap_or_default();
    let status_id = item.status_id.as_deref().unwrap_or_default();
    let page_req = PageRequest::new(item.page_no, item.page_size);
    let result = SysUser::select_page_by_name(rb, &page_req, mobile, status_id).await?;
    let page = Page::<UserListData>::from(result);
    Ok(page)
    
}


// 添加用户信息
pub async fn user_save(item: UserSaveReq) -> Result<u64> {

    let rb = pool!();
    let mut sys_user = SysUser::from(item);
    sys_user.password = Password::md5_and_hash(&sys_user.password);

    let result = SysUser::insert(rb, &sys_user).await?;

    Ok(result.rows_affected)
}

// 更新用户信息
pub async fn user_update(item: UserUpdateReq) -> Result<u64> {

    let rb = pool!();
    let result = SysUser::select_by_id(rb, item.id).await?;

    match result {
        None => {
            Error::err("用户不存在")
        }
        Some(_user) => {
            let result = UserUpdateReq::update_by_column(rb, &item, "id")
                .await?;
            Ok(result.rows_affected)
        }
    }
}

// 删除用户信息
pub async fn user_delete(item: UserDeleteReq) -> Result<u64> {
    let rb = pool!();
    //id为1的用户为系统预留用户,不能删除
    let ids: Vec<u64> = item.ids.par_iter()
        .filter(|x| **x != 1).cloned()
        .collect();

    let result = SysUser::delete_in_column(rb, "id",&ids).await?;
    Ok(result.rows_affected)
}

// 更新用户密码
pub async fn update_user_password(item: UpdateUserPwdReq) -> Result<u64> {
    let rb = pool!();

    let sys_user_result = SysUser::select_by_id(rb, item.id).await?;

    match sys_user_result {
        None => {
            Error::err("用户不存在")
        }
        Some(user) => {
            if Password::verify(&item.password, &user.password) {
                let id = user.id.unwrap();
                let password = Password::hash(&item.new_password);
                let result = SysUser::update_password(rb, id, &password).await;
                info!("update_user_pwd result: {:?}", result);
                if result.is_ok() {
                    Ok(1)
                } else {
                    Error::err("密码修改失败")
                }
            } else {
                Error::err("旧密码不正确")
            }
        }
    }
}