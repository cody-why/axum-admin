DROP TABLE IF EXISTS sys_user;
create table sys_user
(
    id          bigint UNSIGNED auto_increment comment '主键'
        primary key,
    mobile      char(11) default ''                not null comment '手机',
    user_name   varchar(50)                        not null comment '姓名',
    password    varchar(64) charset utf8mb3        null comment '密码',
    status_id   tinyint  default 1                 not null comment '状态(1:正常，0:禁用)',
    sort        int      default 1                 not null comment '排序',
    remark      varchar(255)                       null comment '备注',
    create_time datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间',
    constraint AK_phone
        unique (mobile)
)
    comment '用户信息';

INSERT INTO sys_user (id, mobile, user_name, password, status_id, sort, remark) VALUES (1, '18500000000', '超级用户', '$2b$12$HDdZBmuoaWASJmEncppJrOMmtHxtW2K.RDGzzMc0IGffn6I4gUWQ2', 1, 1, '超级用户');
INSERT INTO sys_user (id, mobile, user_name, password, status_id, sort, remark) VALUES (2, '13800000000', '普通用户', '$2b$12$HDdZBmuoaWASJmEncppJrOMmtHxtW2K.RDGzzMc0IGffn6I4gUWQ2', 1, 1, '演示权限');
