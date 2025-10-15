use chrono::{DateTime, NaiveDateTime, Utc};

use crate::{
    dto::app_error::AppError,
    modules::{self, conditions::schema::Conditions},
};

pub fn map_row_to_condition(row: tokio_postgres::Row) -> Conditions {
    let created_on_datetime: DateTime<Utc> = row.get("created_on");
    let data = Conditions {
        id: row.get("id"),
        location: row.get("location"),
        temperature: row.get("temperature"),
        humidity: row.get("humidity"),
        created_on: created_on_datetime.naive_utc(),
    };

    data
}

pub async fn find_all(
    client: &tokio_postgres::Client,
) -> Result<Vec<modules::conditions::schema::Conditions>, AppError> {
    let statement = "SELECT id, created_on, location, temperature, humidity FROM conditions ORDER BY created_on DESC";
    let rows = client
        .query(statement, &[])
        .await
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    let _conditions: Vec<Conditions> = rows.into_iter().map(map_row_to_condition).collect();
    return Ok(_conditions);
}

pub async fn find_by_id(
    client: &tokio_postgres::Client,
    id: String,
) -> Result<modules::conditions::schema::Conditions, AppError> {
    let statement =
        "SELECT id, created_on, location, temperature, humidity FROM conditions WHERE id=$1";
    let rows = client
        .query(statement, &[&id])
        .await
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    let _conditions: Vec<Conditions> = rows.into_iter().map(map_row_to_condition).collect();
    let _condition = _conditions.first();
    if _condition.is_none() {
        return Err(AppError::NotFound);
    }
    return Ok(_condition.unwrap().clone());
}

pub async fn delete_all(client: &tokio_postgres::Client) -> Result<(), AppError> {
    let statement = "DELETE FROM conditions";

    let rows_affected = client
        .execute(statement, &[])
        .await
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    if rows_affected == 0 {
        return Err(AppError::NotFound);
    }

    return Ok(());
}

pub async fn delete_by_id(client: &tokio_postgres::Client, id: String) -> Result<(), AppError> {
    let statement = "DELETE FROM conditions WHERE id=$1";

    let rows_affected = client
        .execute(statement, &[&id])
        .await
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    if rows_affected == 0 {
        return Err(AppError::NotFound);
    }

    return Ok(());
}

pub async fn insert_all(
    client: &mut tokio_postgres::Client,
    data: Vec<Conditions>,
) -> Result<(), AppError> {
    println!("insert_all");
    let transaction = client
        .transaction()
        .await
        .map_err(|error| AppError::Other(format!("transaction failed: {}", error)))?;

    let statement = "INSERT INTO conditions (id, location, temperature, humidity, created_on) 
                     VALUES ($1, $2, $3, $4, $5)";

    let prepared_statement = transaction
        .prepare(statement)
        .await
        .map_err(|error| AppError::Other(format!("prepare statement failed: {}", error)))?;

    println!("for loop");
    for condition in data.clone() {
        transaction
            .execute(
                &prepared_statement,
                &[
                    &condition.id,
                    &condition.location,
                    &condition.temperature,
                    &condition.humidity,
                    &condition.created_on.and_utc(),
                ],
            )
            .await
            .map_err(|error| AppError::Other(format!("execute failed: {}", error)))?;
    }

    println!("data size: {}", data.len());

    transaction
        .commit()
        .await
        .map_err(|error| AppError::Other(format!("commit failed: {}", error)))?;

    println!("data size: {}", data.len());

    return Ok(());
}

pub async fn insert_one(
    client: &mut tokio_postgres::Client,
    condition: Conditions,
) -> Result<(), AppError> {
    let transaction = client
        .transaction()
        .await
        .map_err(|error| AppError::Other(format!("transaction failed: {}", error)))?;

    let statement = "INSERT INTO conditions (id, location, temperature, humidity, created_on) 
                     VALUES ($1, $2, $3, $4, $5)";

    let prepared_statement = transaction
        .prepare(statement)
        .await
        .map_err(|error| AppError::Other(format!("prepare statement failed: {}", error)))?;

    transaction
        .execute(
            &prepared_statement,
            &[
                &condition.id,
                &condition.location,
                &condition.temperature,
                &condition.humidity,
                &condition.created_on.and_utc(),
            ],
        )
        .await
        .map_err(|error| AppError::Other(format!("execute failed: {}", error)))?;

    transaction
        .commit()
        .await
        .map_err(|error| AppError::Other(format!("commit failed: {}", error)))?;

    return Ok(());
}
