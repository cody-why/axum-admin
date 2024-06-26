use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
use crate::model::menu::SysMenu;
use crate::model::role::SysRole;

#[derive(Debug, Deserialize)]
pub struct RoleListReq {
    #[serde(rename = "current")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    pub role_name: Option<String>,
    pub status_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RoleListData {
    pub id: i32,
    pub sort: i32,
    pub status_id: i32,
    pub role_name: String,
    pub remark: String,
    pub create_time: String,
    pub update_time: String,
}

impl From<SysRole> for RoleListData {
    fn from(role: SysRole) -> Self {
        Self {
            id: role.id.unwrap(),
            sort: role.sort,
            status_id: role.status_id,
            role_name: role.role_name,
            remark: role.remark.unwrap_or_default(),
            create_time: role.create_time.unwrap().to_string(),
            update_time: role.update_time.unwrap().to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RoleSaveReq {
    pub role_name: String,
    pub sort: i32,
    pub status_id: i32,
    pub remark: Option<String>,
}

impl From<RoleSaveReq> for SysRole {
    fn from(role_req: RoleSaveReq) -> Self {
        let now = Some(DateTime::now());
        SysRole {
            id: None,
            sort: role_req.sort,
            status_id: role_req.status_id,
            role_name: role_req.role_name,
            remark: role_req.remark,
            create_time: now.clone(),
            update_time: now,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoleUpdateReq {
    pub id: i32,
    pub sort: i32,
    pub status_id: i32,
    pub role_name: String,
    pub remark: Option<String>,
}

impl_update!(RoleUpdateReq{}, "sys_role");

impl From<RoleUpdateReq> for SysRole {
    fn from(role_req: RoleUpdateReq) -> Self {
        let now = Some(DateTime::now());
        SysRole {
            id: Some(role_req.id),
            sort: role_req.sort,
            status_id: role_req.status_id,
            role_name: role_req.role_name,
            remark: role_req.remark,
            create_time: None,
            update_time: now,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RoleDeleteReq {
    pub ids: Vec<i32>,
}


#[derive(Debug, Deserialize)]
pub struct QueryRoleMenuReq {
    pub role_id: i32,
}

#[derive(Debug, Serialize)]
pub struct QueryRoleMenuData {
    pub role_menus: Vec<i32>,
    pub menu_list: Vec<MenuDataList>,
}

#[derive(Debug, Serialize)]
pub struct MenuDataList {
    pub id: i32,
    pub parent_id: i32,
    pub title: String,
    pub key: String,
    // pub label: String,
    #[serde(rename = "isPenultimate")]
    pub is_penultimate: bool,
}

impl From<SysMenu> for MenuDataList {
    fn from(role: SysMenu) -> Self {
        Self {
            id: role.id.unwrap(),
            parent_id: role.parent_id,
            title: role.menu_name,
            key: role.id.unwrap().to_string(),
            is_penultimate: role.parent_id == 2,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoleMenuReq {
    pub menu_ids: Vec<i32>,
    pub role_id: i32,
}


