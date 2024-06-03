use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
use crate::model::menu::SysMenu;

#[derive(Debug, Deserialize)]
pub struct MenuListReq {
    pub menu_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MenuListData {
    pub id: i32,
    pub sort: i32,
    pub status_id: i32,
    pub parent_id: i32,
    pub menu_name: String,
    // pub label: String,
    pub menu_url: String,
    pub icon: String,
    pub api_url: String,
    pub remark: String,
    pub menu_type: i32,
    pub create_time: String,
    pub update_time: String,
}

impl From<SysMenu> for MenuListData {
    fn from(menu: SysMenu) -> Self {
        Self {
            id: menu.id.unwrap(),
            sort: menu.sort,
            status_id: menu.status_id,
            parent_id: menu.parent_id,
            menu_name: menu.menu_name,
            menu_url: menu.menu_url.unwrap_or_default(),
            icon: menu.menu_icon.unwrap_or_default(),
            api_url: menu.api_url.unwrap_or_default(),
            remark: menu.remark.unwrap_or_default(),
            menu_type: menu.menu_type,
            create_time: menu.create_time.unwrap().to_string(),
            update_time: menu.update_time.unwrap().to_string(),
        }
    }
}

#[test]
fn test() {
    let time = DateTime::now();
    println!("{:?}", time.to_string());
    println!("{:?}", time.0.to_string())
}

#[derive(Debug, Deserialize)]
pub struct MenuSaveReq {
    pub sort: i32,
    pub status_id: i32,
    pub parent_id: Option<i32>,
    pub menu_name: String,
    pub menu_url: Option<String>,
    pub icon: Option<String>,
    pub api_url: Option<String>,
    pub remark: Option<String>,
    pub menu_type: i32,
}

impl From<MenuSaveReq> for SysMenu {
    fn from(req: MenuSaveReq) -> Self {
        let now = Some(DateTime::now());
        Self {
            id: None,
            sort: req.sort,
            status_id: req.status_id,
            parent_id: req.parent_id.unwrap_or_default(),
            menu_name: req.menu_name,
            menu_url: req.menu_url,
            menu_icon: req.icon,
            api_url: req.api_url,
            remark: req.remark,
            menu_type: req.menu_type,
            create_time: now.clone(),
            update_time: now,
        }
    }
}

#[derive(Debug,Serialize, Deserialize)]
pub struct MenuUpdateReq {
    pub id: i32,
    pub sort: i32,
    pub status_id: i32,
    pub parent_id: i32,
    pub menu_name: String,
    pub menu_url: Option<String>,
    #[serde(rename = "menu_icon")]
    pub icon: Option<String>,
    pub api_url: Option<String>,
    pub remark: Option<String>,
    pub menu_type: i32,
}

impl_update!(MenuUpdateReq{}, "sys_menu");
impl From<MenuUpdateReq> for SysMenu {
    fn from(req: MenuUpdateReq) -> Self {
        let now = Some(DateTime::now());
        Self {
            id: Some(req.id),
            sort: req.sort,
            status_id: req.status_id,
            parent_id: req.parent_id,
            menu_name: req.menu_name,
            menu_url: req.menu_url,
            menu_icon: req.icon,
            api_url: req.api_url,
            remark: req.remark,
            menu_type: req.menu_type,
            create_time: None,
            update_time: now,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MenuDeleteReq {
    pub id: i32,
}
