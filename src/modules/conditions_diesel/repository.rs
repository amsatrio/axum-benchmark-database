use diesel::sql_query;
use diesel::{
    ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
    dsl::update, insert_into,
};

use crate::modules::conditions_diesel::schema::CountResult;
use crate::schema::conditions::dsl::*;
use crate::{
    dto::app_error::AppError, modules::conditions_diesel::schema::Conditions, schema::conditions::id,
};

pub fn find_by_id(
    conn: &mut PgConnection,
    conditions_id: String,
) -> Result<Option<Conditions>, AppError> {
    let user = conditions
        .filter(id.eq(conditions_id.to_owned()))
        .select(Conditions::as_select())
        .first::<Conditions>(conn)
        .optional()
        .map_err(|error| {
            AppError::Other(format!("query failed: {}, id: {}", error, conditions_id))
        })?;

    Ok(user)
}

pub fn find_all_by_query(conn: &mut PgConnection) -> Result<Vec<Conditions>, AppError> {
    let query = "SELECT id,created_on,temperature,location,humidity
            FROM conditions";

    let user: Vec<Conditions> = sql_query(query)
        .get_results::<Conditions>(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    Ok(user)
}

pub fn find_all(conn: &mut PgConnection) -> Result<Vec<Conditions>, AppError> {
    let user: Vec<Conditions> = conditions
        .select(Conditions::as_select()).limit(1000000)
        .load(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    Ok(user)
}

pub fn find_all_by_query_pagination(conn: &mut PgConnection) -> Result<Vec<Conditions>, AppError> {
    let mut data: Vec<Conditions> = Vec::new();

    let query_count = "SELECT COUNT(id) as count FROM conditions";

    let count = sql_query(query_count)
        .get_results::<CountResult>(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;

    let size = 100;
    let mut total_of_pages = count[0].count / size;
    if count[0].count % size != 0 {
        total_of_pages = total_of_pages + 1;
    }

    for page in 0..total_of_pages {
        let query_pagination = format!("LIMIT {} OFFSET {}", size, size * (page));
        let query = format!(
            "{} {}",
            "SELECT id,created_on,temperature,location,humidity
            FROM conditions",
            query_pagination
        );

        let mut result = sql_query(query)
            .get_results::<Conditions>(conn)
            .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
        data.append(&mut result);
    }
    Ok(data)
}

pub fn delete_by_id(
    conn: &mut PgConnection,
    conditions_id: String,
) -> Result<Option<()>, AppError> {
    let rows_affected = diesel::delete(conditions.filter(id.eq(conditions_id.to_owned())))
        .execute(conn)
        .map_err(|error| {
            AppError::Other(format!("query failed: {}, id: {}", error, conditions_id))
        })?;

    if rows_affected > 0 {
        return Ok(Some(()));
    }
    return Ok(None);
}

pub fn delete_all(conn: &mut PgConnection) -> Result<Option<()>, AppError> {
    let rows_affected = diesel::delete(conditions)
        .execute(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;

    if rows_affected > 0 {
        return Ok(Some(()));
    }
    return Ok(None);
}

pub fn create(conn: &mut PgConnection, data: Conditions) -> Result<Option<()>, AppError> {
    let rows_affected = insert_into(conditions)
        .values(&data)
        .execute(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    if rows_affected > 0 {
        return Ok(Some(()));
    }
    return Ok(None);
}

pub fn create_bacth(
    conn: &mut PgConnection,
    data: Vec<Conditions>,
) -> Result<Option<()>, AppError> {
    let rows_affected = insert_into(conditions)
        .values(&data)
        .execute(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}", error)))?;
    if rows_affected > 0 {
        return Ok(Some(()));
    }
    return Ok(None);
}

pub fn update_data(conn: &mut PgConnection, data: Conditions) -> Result<Option<()>, AppError> {
    let rows_affected = update(conditions.filter(id.eq(data.id.to_owned())))
        .set((
            location.eq(data.location),
            temperature.eq(data.temperature),
            humidity.eq(data.humidity),
        ))
        .execute(conn)
        .map_err(|error| AppError::Other(format!("query failed: {}, id: {}", error, data.id)))?;
    if rows_affected > 0 {
        return Ok(Some(()));
    }
    return Ok(None);
}
