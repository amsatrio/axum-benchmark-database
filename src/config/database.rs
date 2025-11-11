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

pub async fn get_tiberius_sql_server() -> Result<tiberius::Client<Compat<TcpStream>>, tiberius::error::Error> {
    let mut config = tiberius::Config::new();
    config.host("localhost");
    config.port(1433);
    config.authentication(tiberius::AuthMethod::sql_server("SA", "P@ssw0rd"));
    config.trust_cert();
    
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;
    let client = tiberius::Client::connect(config, tcp.compat_write()).await?;

    return Ok(client);
}
