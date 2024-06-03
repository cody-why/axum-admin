use rbatis::rbatis::RBatis;

pub async fn init_db() -> RBatis {
    let url = include_str!("../../.env").trim_start_matches("DATABASE_URL=");
    let rb = RBatis::new();
    rb.init(rbdc_mysql::driver::MysqlDriver {}, url).unwrap();
    let pool = rb.get_pool().unwrap();
    pool.set_max_open_conns(10).await;
    rb
}