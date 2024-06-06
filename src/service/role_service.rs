use crate::{pool, Result};
use rbatis::plugin::page::PageRequest;
use rbatis::Page;
use log::info;
use crate::model::menu::SysMenu;
use crate::model::role::SysRole;
use crate::model::role_menu::{query_menu_by_role, SysRoleMenu};
use crate::model::user_role::SysUserRole;
use crate::vo::role_vo::*;

// 查询角色列表
pub async fn role_list(item: RoleListReq) -> Result<Page<RoleListData>> {
    info!("role_list params: {:?}", &item);
    let rb = pool!();

    let role_name = item.role_name.as_deref().unwrap_or_default();
    let status_id = item.status_id.as_deref().unwrap_or_default();

    let page_req = PageRequest::new(item.page_no, item.page_size);
    let result = SysRole::select_page_by_name(rb, &page_req, role_name, status_id).await?;

    let page = Page::<RoleListData>::from(result);
    Ok(page)

}


// 添加角色信息
pub async fn role_save(item: RoleSaveReq) -> Result<u64> {
    let rb = pool!();

    let sys_role = SysRole::from(item);
    let result = SysRole::insert(rb, &sys_role).await?;
    Ok(result.rows_affected)
}

// 更新角色信息
pub async fn role_update(item: RoleUpdateReq) -> Result<u64> {
    info!("role_update params: {:?}", &item);
    let rb = pool!();

    // let sys_role = SysRole::from(item);
    let result = RoleUpdateReq::update_by_column(rb, &item, "id").await?;
    Ok(result.rows_affected)
}

// 删除角色信息
pub async fn role_delete(item: RoleDeleteReq) -> Result<u64>  {
    let rb = pool!();

    let ids = item.ids.clone();
    let user_role_list = SysUserRole::select_in_column(rb, "role_id", &ids).await?;

    if !user_role_list.is_empty() {
        return Err("角色已被使用,不能直接删除".into());
    }
    let result = SysRole::delete_in_column(rb, "id", &item.ids).await?;
    Ok(result.rows_affected)
}

// 查询角色关联的菜单
pub async fn query_role_menu(item: QueryRoleMenuReq) -> Result<QueryRoleMenuData> {
    info!("query_role_menu params: {:?}", &item);
    let rb = pool!();

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
    Ok(result)
}

// 更新角色关联的菜单
pub async fn update_role_menu(item: UpdateRoleMenuReq) -> Result<u64> {
    info!("update_role_menu params: {:?}", &item);
    let role_id = item.role_id;

    let rb = pool!();

    let _ = SysRoleMenu::delete_by_column(rb, "role_id", &role_id).await?;

    let mut menu_role: Vec<SysRoleMenu> = Vec::with_capacity(item.menu_ids.len());

    for id in &item.menu_ids {
        let menu_id = *id;
        menu_role.push(SysRoleMenu::new(role_id, menu_id))
    }

    let result = SysRoleMenu::insert_batch(rb, &menu_role, item.menu_ids.len() as u64).await?;
    Ok(result.rows_affected)
}
