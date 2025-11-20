use futures_util::StreamExt;
use tiberius::{IntoRow, QueryItem};

use crate::{dto::app_error::AppError, modules::conditions_tiberius_columns::schema::Conditions};

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
        .bulk_insert("master.dbo.conditions")
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
