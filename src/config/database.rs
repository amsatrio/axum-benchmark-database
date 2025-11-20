use std::time::Duration;

use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
use diesel::{PgConnection, r2d2};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};
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

pub async fn get_tokio_postgresql() -> Result<tokio_postgres::Client, tokio_postgres::Error> {
    let config_env = &CONFIG;
    let database_url = config_env.get_database_url();
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("database connection error: {}", e);
        }
    });
    return Ok(client);
}

pub fn get_deadpool_tiberius_sql_server_db_pool() -> deadpool_tiberius::Pool {
    let pool = deadpool_tiberius::Manager::new()
        .host("localhost")
        .port(1433)
        .basic_authentication("sa", "P@ssw0rd")
        .database("master")

        .trust_cert()
        .max_size(17)
        .wait_timeout(Duration::from_secs(30))  
        .pre_recycle_sync(|_client, _metrics| {
            Ok(())
        })
        .create_pool();

    pool.unwrap()
}