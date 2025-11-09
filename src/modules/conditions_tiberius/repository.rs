use chrono::FixedOffset;
use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use tiberius::{QueryItem, ToSql};
use tokio::time::Instant;

use crate::{dto::app_error::AppError, modules::conditions_tiberius::schema::Conditions};

pub fn map_row_to_condition(option_row: std::option::Option<tiberius::Row>) -> Conditions {
    let row = option_row.unwrap();
    let created_on_datetime: DateTime<FixedOffset> = row.get("created_on").unwrap();
    Conditions {
        id: row.get::<&str, _>("id").unwrap_or_default().to_string(),
        location: row
            .get::<&str, _>("location")
            .unwrap_or_default()
            .to_string(),
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
    // println!("{}", duration.as_millis());

    let conditions: Vec<Conditions> = rows
        .into_row()
        .await
        .into_par_iter()
        .map(|option_row| {
            if option_row.is_none() {
                return Err(());
            }
            Ok(map_row_to_condition(option_row))
        })
        .filter_map(Result::ok)
        .collect();

    return Ok(conditions);
}

pub async fn find_all_stream(
    client: &mut tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>,
) -> Result<Vec<Conditions>, AppError> {
    let statement = "SELECT * FROM conditions";
    let mut stream = client
        .query(statement, &[])
        .await
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;

    let mut conditions = Vec::new();

    while let Some(item) = stream
        .next()
        .await
        .transpose()
        .map_err(|err| AppError::Other(format!("{:?}", err)))?
    {
        if let QueryItem::Row(row) = item {
            let condition = Conditions::from_row_tiberius(&row);
            conditions.push(condition);
        }
    }

    Ok(conditions)
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
    // println!("{}", duration.as_millis());

    if execute_result.rows_affected()[0] == 0 {
        return Err(AppError::NotFound);
    }

    return Ok(());
}

pub async fn delete_by_id(
    client: &mut tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>,
    id: String,
) -> Result<(), AppError> {
    let start = Instant::now();
    let statement = "DELETE FROM conditions where @P1";
    let execute_result = client
        .execute(statement, &[&id])
        .await
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    let duration = start.elapsed();
    // println!("{}", duration.as_millis());

    if execute_result.rows_affected()[0] == 0 {
        return Err(AppError::NotFound);
    }

    return Ok(());
}

pub async fn insert_batch(
    client: &mut tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>,
    data: Vec<Conditions>,
) -> Result<(), AppError> {
    let mut query = String::from(
        "INSERT INTO conditions (id, created_on, location, temperature, humidity) VALUES ",
    );

    let mut params: Vec<Box<dyn ToSql>> = Vec::new();
    for condition in &data {
        let values = format!(
            "(@p{}, @p{}, @p{}, @p{}, @p{})",
            params.len() + 1,
            params.len() + 2,
            params.len() + 3,
            params.len() + 4,
            params.len() + 5
        );
        query.push_str(&values);
        query.push_str(", ");

        params.push(Box::new(condition.id.clone()));
        params.push(Box::new(condition.created_on));
        params.push(Box::new(condition.location.clone()));
        params.push(Box::new(
            condition
                .temperature
                .as_ref()
                .map(|v| *v)
                .unwrap_or_default(),
        ));
        params.push(Box::new(
            condition.humidity.as_ref().map(|v| *v).unwrap_or_default(),
        ));
    }

    query.truncate(query.len() - 2);

    let params_slice: Vec<&(dyn tiberius::ToSql)> = params.iter().map(|p| p.as_ref()).collect();

    client
        .execute(&query, &params_slice)
        .await
        .map_err(|err| AppError::Other(format!("{:?}", err)))?;
    return Ok(());
}

pub async fn insert_batch_2(
    client: &mut tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>,
    data: Vec<Conditions>,
) -> Result<(), AppError> {
    let mut query = String::from(
        "INSERT INTO conditions (id, created_on, location, temperature, humidity) VALUES ",
    );

    client
        .execute("BEGIN TRANSACTION", &[])
        .await
        .map_err(|err| AppError::Other(format!("{:?}", err)))?;

    let mut params: Vec<Box<dyn ToSql>> = Vec::new();
    for condition in &data {
        let values = format!(
            "(@p{}, @p{}, @p{}, @p{}, @p{})",
            params.len() + 1,
            params.len() + 2,
            params.len() + 3,
            params.len() + 4,
            params.len() + 5
        );
        query.push_str(&values);
        query.push_str(", ");

        params.push(Box::new(condition.id.clone()));
        params.push(Box::new(condition.created_on));
        params.push(Box::new(condition.location.clone()));
        params.push(Box::new(
            condition
                .temperature
                .as_ref()
                .map(|v| *v)
                .unwrap_or_default(),
        ));
        params.push(Box::new(
            condition.humidity.as_ref().map(|v| *v).unwrap_or_default(),
        ));
    }

    query.truncate(query.len() - 2);

    let params_slice: Vec<&(dyn tiberius::ToSql)> = params.iter().map(|p| p.as_ref()).collect();

    client
        .execute(&query, &params_slice)
        .await
        .map_err(|err| AppError::Other(format!("{:?}", err)))?;

    client
        .execute("COMMIT", &[])
        .await
        .map_err(|err| AppError::Other(format!("{:?}", err)))?;

    return Ok(());
}
