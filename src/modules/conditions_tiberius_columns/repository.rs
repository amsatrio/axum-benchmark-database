use std::thread;

use futures_util::{StreamExt, future::join_all};
use tiberius::{IntoRow, QueryItem};

use crate::{dto::app_error::AppError, modules::{conditions, conditions_tiberius_columns::schema::Conditions}};

pub async fn delete_all(
    client: &mut tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>,
) -> Result<(), AppError> {
    let statement = "DELETE FROM conditions";
    let execute_result = client
        .execute(statement, &[])
        .await
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;

    if execute_result.rows_affected()[0] == 0 {
        return Err(AppError::NotFound);
    }

    return Ok(());
}

pub async fn count_data(client: &mut deadpool_tiberius::deadpool::managed::Object<deadpool_tiberius::Manager>,) -> Result<i32, AppError> {
    let where_condition = "created_on BETWEEN '2023-01-01' and '2023-12-31'";
    let statement_count = format!("select count(id) as count_data from conditions where {}", where_condition);

    let mut total_data = 0;
    let mut stream = client
        .query(statement_count, &[])
        .await
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    while let Some(item) = stream
        .next()
        .await
        .transpose()
        .map_err(|err| AppError::Other(format!("{:?}", err)))?
    {
        if let QueryItem::Row(row) = item {
            total_data = row.get("count_data").unwrap();
        }
    }
    if total_data == 0 {
        return Err(AppError::NotFound);
    }
    Ok(total_data)
}


pub async fn find_all_stream_pagination(
    client: &mut deadpool_tiberius::deadpool::managed::Object<deadpool_tiberius::Manager>,
    offset: i32, limit: i32
) -> Result<Vec<Conditions>, AppError> {
    let statement = format!("SELECT * FROM conditions where created_on BETWEEN '2023-01-01' and '2023-12-31' order by created_on OFFSET {} ROWS FETCH NEXT {} ROWS ONLY", offset, limit);
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

pub async fn find_all_stream(
    client: &mut deadpool_tiberius::deadpool::managed::Object<deadpool_tiberius::Manager>,
) -> Result<Vec<Conditions>, AppError> {
    let statement = "SELECT * FROM conditions where created_on BETWEEN '2023-01-01' and '2023-12-31'";
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

pub async fn insert_batch(
    client: &mut tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>,
    data: Vec<Conditions>,
) -> Result<(), AppError> {
    if data.is_empty() {
        return Ok(());
    }
    let mut bulk_insert = client
        .bulk_insert("tiberius.dbo.conditions")
        .await
        .map_err(|err| AppError::Other(format!("{:?}", err)))?;

    for condition in &data {
        bulk_insert
            .send(Conditions::to_row_tiberius(condition))
            .await
            .map_err(|err| AppError::Other(format!("{:?}", err)))?;
    }

    let res = bulk_insert
        .finalize()
        .await
        .map_err(|err| AppError::Other(format!("{:?}", err)))?;
    println!("Result: {:?}", res);
    return Ok(());
}
