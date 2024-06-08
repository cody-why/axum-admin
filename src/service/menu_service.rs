use log::info;

use crate::Error;
use crate::Result;
use crate::pool;
use crate::model::menu::SysMenu;
use crate::vo::menu_vo::*;


pub async fn menu_list() -> Result<Vec<MenuListData>> {
    let rb = pool!();
    // 菜单是树形结构不需要分页
    let result = SysMenu::select_all(rb).await?;
    let menu_list: Vec<MenuListData> = result.into_iter().map(MenuListData::from).collect();
    Ok(menu_list)
    
}


// 添加菜单
pub async fn menu_save(item: MenuSaveReq) -> Result<u64> {
    let rb = pool!();

    let sys_menu = SysMenu::from(item);

    let result = SysMenu::insert(rb, &sys_menu).await?;

    Ok(result.rows_affected)
}

// 更新菜单
pub async fn menu_update(item: MenuUpdateReq) -> Result<u64> {
    info!("menu_update params: {:?}", &item);
    let rb = pool!();
    let sys_menu = SysMenu::from(item);
    let result = SysMenu::update_by_column(rb, &sys_menu, "id").await?;

    Ok(result.rows_affected)
}

// 删除菜单信息
pub async fn menu_delete(item: MenuDeleteReq) -> Result<u64> {
    info!("menu_delete params: {:?}", &item);
    let rb = pool!();
    let mut count = 0;
    for id in item.ids {
        //有下级的时候 不能直接删除
        let menus = SysMenu::select_by_column(rb, "parent_id", &id).await.unwrap_or_default();
        if !menus.is_empty() {
            return Error::err("有下级菜单,不能直接删除")
        }
        let result = SysMenu::delete_by_column(rb, "id", &id).await?;
        count += result.rows_affected;
    }
    Ok(count)
}