DROP TABLE IF EXISTS sys_role_menu;
create table sys_role_menu
(
    id          int auto_increment comment '主键'
        primary key,
    role_id     int                             not null comment '角色ID',
    menu_id     int                             not null comment '菜单ID',
    status_id   tinyint  default 1                 not null comment '状态(1:正常，0:禁用)',
    sort        int      default 1                 not null comment '排序',
    create_time datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间',
    -- 关联role表
    foreign key (role_id) references sys_role(id)
        on delete cascade,
    -- 关联menu表
    foreign key (menu_id) references sys_menu(id)
        on delete cascade,
    -- 联合唯一索引
    unique key idx_role_menu (role_id, menu_id)
)
    comment '菜单角色关联表';

INSERT INTO sys_role_menu (role_id, menu_id, status_id, sort) VALUES (3, 1, 1, 1);
INSERT INTO sys_role_menu (role_id, menu_id, status_id, sort) VALUES (3, 17, 1, 1);
INSERT INTO sys_role_menu (role_id, menu_id, status_id, sort) VALUES (3, 65, 1, 1);
INSERT INTO sys_role_menu (role_id, menu_id, status_id, sort) VALUES (3, 66, 1, 1);
INSERT INTO sys_role_menu (role_id, menu_id, status_id, sort) VALUES (3, 67, 1, 1);
INSERT INTO sys_role_menu (role_id, menu_id, status_id, sort) VALUES (3, 68, 1, 1);
INSERT INTO sys_role_menu (role_id, menu_id, status_id, sort) VALUES (3, 69, 1, 1);
INSERT INTO sys_role_menu (role_id, menu_id, status_id, sort) VALUES (3, 72, 1, 1);
INSERT INTO sys_role_menu (role_id, menu_id, status_id, sort) VALUES (3, 73, 1, 1);
