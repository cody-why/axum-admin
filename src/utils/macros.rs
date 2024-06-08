
/// erverwhere use the `context!` macro to get a reference to the `ServiceContext` struct.
#[macro_export]
macro_rules! context {
    () => {
        &$crate::service::CONTEXT
    };
}

/// erverwhere use the `pool!` macro to get a reference to the `RBatis` pool.
#[macro_export]
macro_rules! pool {
    () => {
        &$crate::service::CONTEXT.rb
    };
}



#[macro_export]
macro_rules! redis_conn {
    () => {
        async {
            let pool = $crate::utils::redis::get_redis_pool().await;
            pool.get().await
        }.await
    };
}
