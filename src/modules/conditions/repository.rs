use bytes::BytesMut;
use chrono::{DateTime, Utc};
use csv::WriterBuilder;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    dto::app_error::AppError,
    modules::{self, conditions::schema::Conditions},
};
use futures_util::{pin_mut, sink::SinkExt};
use tempfile::Builder;
use tokio::{fs::File as AsyncFile, io::AsyncReadExt, time::Instant};

pub fn map_row_to_condition(row: tokio_postgres::Row) -> Conditions {
    let created_on_datetime: DateTime<Utc> = row.get("created_on");
    Conditions {
        id: row.get("id"),
        location: row.get("location"),
        temperature: row.get("temperature"),
        humidity: row.get("humidity"),
        created_on: created_on_datetime.naive_utc(),
    }
}

pub async fn find_all(
    client: &tokio_postgres::Client,
) -> Result<Vec<modules::conditions::schema::Conditions>, AppError> {
    let start = Instant::now();
    let statement = "SELECT id, created_on, location, temperature, humidity FROM conditions";
    let rows = client
        .query(statement, &[])
        .await
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    let duration = start.elapsed();
    println!("{}", duration.as_millis());
    return Ok(rows.into_par_iter().map(map_row_to_condition).collect());
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

    transaction
        .commit()
        .await
        .map_err(|error| AppError::Other(format!("commit failed: {}", error)))?;

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

pub async fn insert_batch(
    client: &mut tokio_postgres::Client,
    data: Vec<Conditions>,
) -> Result<(), AppError> {
    // 1. Start a transaction (unchanged)
    let transaction = client
        .transaction()
        .await
        .map_err(|error| AppError::Other(format!("transaction failed: {}", error)))?;

    // 2. Define the COPY statement (unchanged)
    let copy_statement = "COPY conditions (id, created_on, location, temperature, humidity) FROM STDIN WITH (FORMAT csv)";

    // 3-5. Write to a temporary file (unchanged)
    let temp_file = Builder::new()
        .prefix("conditions_batch_")
        .suffix(".csv")
        .tempfile()
        .map_err(|e| AppError::Other(format!("Failed to create temp file: {}", e)))?;

    let std_file = temp_file
        .reopen()
        .map_err(|e| AppError::Other(format!("Failed to reopen temp file: {}", e)))?;

    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_writer(std_file);

    for condition in data {
        wtr.serialize(condition)
            .map_err(|error| AppError::Other(format!("CSV serialization failed: {}", error)))?;
    }

    wtr.flush()
        .map_err(|error| AppError::Other(format!("CSV flush failed: {}", error)))?;

    drop(wtr);

    let set_tz_statement = "SET TIME ZONE 'Asia/Jakarta'";
    transaction
        .execute(set_tz_statement, &[])
        .await
        .map_err(|error| AppError::Other(format!("SET TIME ZONE failed: {}", error)))?;

    // 6. Initiate the COPY operation and get the sink
    let sink = transaction
        .copy_in(copy_statement)
        .await
        .map_err(|error| AppError::Other(format!("copy_in failed: {}", error)))?;

    pin_mut!(sink);

    // 7. Open the temporary file asynchronously for reading
    let mut async_file = AsyncFile::open(temp_file.path())
        .await
        .map_err(|e| AppError::Other(format!("Failed to open file for read: {}", e)))?;

    // let path = temp_file.path().to_owned();
    // println!("Temporary file kept for debugging at: {}", path.display());
    // let persistent_file = temp_file.keep();

    // -----------------------------------------------------
    // 🔑 Fix: Manually read in chunks and send to the Sink
    // -----------------------------------------------------

    // Define a buffer size (e.g., 64KB chunk size is common)
    const BUFFER_SIZE: usize = 65536;
    let mut buffer = BytesMut::with_capacity(BUFFER_SIZE);

    loop {
        // SAFETY: We ensure we read no more than the capacity
        let read_len = async_file
            .read_buf(&mut buffer)
            .await
            .map_err(|e| AppError::Other(format!("Failed to read file chunk: {}", e)))?;

        // 9. If read_len is 0, we've reached EOF
        if read_len == 0 {
            break;
        }

        // Send the chunk of data to the sink
        let chunk = buffer.split().freeze();
        sink.send(chunk)
            .await
            .map_err(|error| AppError::Other(format!("Failed to send data chunk: {}", error)))?;

        // Ensure the buffer capacity is reset for the next read
        buffer.reserve(BUFFER_SIZE);
    }

    // 10. Close the sink to signal the end of the data stream to PostgreSQL
    sink.close()
        .await
        .map_err(|error| AppError::Other(format!("Failed to close sink: {}", error)))?;

    // 11. Commit the transaction
    transaction
        .commit()
        .await
        .map_err(|error| AppError::Other(format!("commit failed: {}", error)))?;

    Ok(())
}

pub async fn update_one(
    client: &mut tokio_postgres::Client,
    condition: Conditions,
) -> Result<(), AppError> {
    let transaction = client
        .transaction()
        .await
        .map_err(|error| AppError::Other(format!("transaction failed: {}", error)))?;

    let statement = "UPDATE conditions SET location=$2, temperature=$3, humidity=$4 WHERE id=$1";

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
