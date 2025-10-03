use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
use diesel::{r2d2, PgConnection};
use tokio_postgres::NoTls;

use crate::config::environment::CONFIG;

pub fn get_diesel_postgres_db_pool() -> r2d2::Pool<r2d2::ConnectionManager<PgConnection>> {
    let config_env = &CONFIG;
    let database_url = config_env.get_database_url();
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .max_size(config_env.database_max_pool)
        .min_idle(Some(config_env.database_min_pool))
        .idle_timeout(Some(std::time::Duration::from_secs(10)))
        .build(manager)
        .expect("connection db failed")
}

pub fn get_tokio_postgres_db_pool() -> deadpool_postgres::Pool {
    let config_env = &CONFIG;
    
    let mut cfg = Config::new();
    cfg.dbname = Some(config_env.database_dbname.clone());
    cfg.user = Some(config_env.database_username.clone()); 
    cfg.password = Some(config_env.database_password.clone());
    cfg.host = Some(config_env.database_host.clone()); 

    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    pool
}