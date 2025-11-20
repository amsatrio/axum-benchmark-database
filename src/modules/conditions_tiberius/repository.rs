use futures_util::StreamExt;
use tiberius::{IntoRow, QueryItem, ToSql};

use crate::{dto::app_error::AppError, modules::conditions_tiberius::schema::Conditions};

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
    // let start = Instant::now();
    let statement = "DELETE FROM conditions";
    let execute_result = client
        .execute(statement, &[])
        .await
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    // let duration = start.elapsed();
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
    // let start = Instant::now();
    let statement = "DELETE FROM conditions where @P1";
    let execute_result = client
        .execute(statement, &[&id])
        .await
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    // let duration = start.elapsed();
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
        "INSERT INTO conditions_partition (id, created_on, location, temperature, humidity) VALUES ",
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
        params.push(Box::new(condition.temperature));
        params.push(Box::new(condition.humidity));
    }

    query.truncate(query.len() - 2);

    let params_slice: Vec<&dyn tiberius::ToSql> = params.iter().map(|p| p.as_ref()).collect();

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
    let mut bulk_insert = client
        .bulk_insert("master.dbo.conditions")
        .await
        .map_err(|err| AppError::Other(format!("{:?}", err)))?;

    for condition in &data {
        bulk_insert
            .send(Conditions::to_tiberius_row(condition))
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
