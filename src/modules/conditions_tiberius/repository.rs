use chrono::FixedOffset;
use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelIterator;
use chrono::{DateTime, Utc};
use tokio::time::Instant;

use crate::{dto::app_error::AppError, modules::{conditions_tiberius::schema::Conditions}};




pub fn map_row_to_condition(option_row: std::option::Option<tiberius::Row>) -> Conditions {
    let row = option_row.unwrap();
    let created_on_datetime: DateTime<FixedOffset> = row
        .get("created_on").unwrap();
    Conditions {
        id: row.get::<&str, _>("id").unwrap_or_default().to_string(),
        location: row.get::<&str, _>("location").unwrap_or_default().to_string(),
        temperature: row.get("temperature"),
        humidity: row.get("humidity"),
        created_on: created_on_datetime.with_timezone(&Utc).naive_utc(),
    }
}

pub async fn find_all(
    client: &mut tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>,
) -> Result<Vec<Conditions>, AppError> {
    let start = Instant::now();
    let statement = "SELECT * FROM conditions";
    let rows = client
        .query(statement, &[])
        .await
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    let duration = start.elapsed();
    println!("{}", duration.as_millis());

    let conditions: Vec<Conditions> = rows.into_row().await.into_par_iter().map(|option_row|{
        if option_row.is_none() {
            return Err(());
        }
        Ok(map_row_to_condition(option_row))
    }).filter_map(Result::ok).collect();

    return Ok(conditions);
}

pub async fn delete_all(
    client: &mut tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>,
) -> Result<(), AppError> {
    let start = Instant::now();
    let statement = "DELETE FROM conditions";
    let execute_result = client
        .execute(statement, &[])
        .await
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    let duration = start.elapsed();
    println!("{}", duration.as_millis());

    if execute_result.rows_affected()[0] == 0 {
        return Err(AppError::NotFound);
    }

    return Ok(());
}

pub async fn delete_by_id(
    client: &mut tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>,
    id: String
) -> Result<(), AppError> {
    let start = Instant::now();
    let statement = "DELETE FROM conditions where @P1";
    let execute_result = client
        .execute(statement, &[&id])
        .await
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    let duration = start.elapsed();
    println!("{}", duration.as_millis());

    if execute_result.rows_affected()[0] == 0 {
        return Err(AppError::NotFound);
    }

    return Ok(());
}