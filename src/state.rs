use std::sync::Arc;

use diesel::{r2d2, PgConnection};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_util::compat::Compat;

pub struct AppState {
    pub diesel_pool_pg: Arc<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>>,
    pub pool_pg: deadpool_postgres::Pool,
    pub tokio_postgres_client: Mutex<tokio_postgres::Client>,
    pub tiberius_client: Mutex<tiberius::Client<Compat<TcpStream>>>,
    pub status: String
}
