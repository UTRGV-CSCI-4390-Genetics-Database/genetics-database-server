use crate::util::{self, Array2SerializeWrapper};
use anyhow::format_err;
use deadpool_postgres::Pool;
use indoc::indoc;
use itertools::Itertools;
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use serde_json as json;
use std::{convert::Infallible, str};
use tokio_postgres::{SimpleQueryMessage, SimpleQueryRow};
use warp::{http::StatusCode, Filter, Rejection, Reply};
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
struct Params {
    pub query: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
struct Response<'a, 'b> {
    pub columnNames: Vec<&'a str>,
    pub columnTypes: Vec<&'b str>,
    pub rows: Array2SerializeWrapper<json::Value>,
}

#[derive(Debug, thiserror::Error)]
enum RequestError {
    #[error(transparent)]
    InvalidInput(#[from] str::Utf8Error),

    #[error(transparent)]
    SqlError(#[from] tokio_postgres::error::DbError),

    #[error(transparent)]
    ServerError(#[from] anyhow::Error),
}

pub fn create(
    pool: Pool,
) -> impl Filter<Extract = (warp::reply::Response,), Error = Rejection> + Clone {
    warp::query().and_then(move |params: Params| {
        let pool = pool.clone();
        async move {
            let result = execute_query(&pool, &params.query).await;

            handle_errors(result)
        }
    })
}

async fn execute_query(pool: &Pool, query: &str) -> Result<warp::reply::Json, RequestError> {
    let mut db = pool.get().await.map_err(anyhow::Error::new)?;
    let db = db.transaction().await?;

    let mut setup_query = indoc!(
        "
        CREATE TEMP TABLE tmp
            ON COMMIT DROP
        AS
        "
    )
    .to_owned();

    let decoded_query = percent_encoding::percent_decode_str(query).decode_utf8()?;
    setup_query.push_str(&decoded_query);
    db.batch_execute(&setup_query).await?;

    let metadata_query = indoc!(
        "
        SELECT column_name, data_type
        FROM information_schema.columns
        WHERE table_name = 'tmp'
        ORDER BY ordinal_position"
    );

    let metadata_results = first_query_rows(db.simple_query(metadata_query).await?);

    let column_names: Option<Vec<_>> = metadata_results.iter().map(|row| row.get(0)).collect();
    let column_names =
        column_names.ok_or_else(|| format_err!("database returned null column name"))?;

    let column_types: Option<Vec<_>> = metadata_results.iter().map(|row| row.get(1)).collect();
    let column_types =
        column_types.ok_or_else(|| format_err!("database returned null column type"))?;

    let row_query = indoc!(
        "
        SELECT *
        FROM tmp"
    );

    let rows = first_query_rows(db.simple_query(row_query).await?);
    let row_count = rows.len();
    let rows: Result<Vec<_>, RequestError> = rows
        .iter()
        .flat_map(|row| {
            column_types.iter().enumerate().map(move |(i, column_type)| {
                let x = if let Some(y) = row.get(i) {
                    y
                } else {
                    return Ok(json::Value::Null);
                };

                let x = match *column_type {
                    "boolean" => json::Value::from(if x == "t" { true } else { false }),
                    "integer" | "smallint" | "bigint" => {
                        json::Value::from(x.parse::<f64>().map_err(anyhow::Error::new)?)
                    }
                    _ => json::Value::from(x),
                };

                Ok(x)
            })
        })
        .collect();
    let rows = Array2::from_shape_vec((row_count, column_names.len()), rows?).unwrap();

    let response = Response {
        columnNames: column_names,
        columnTypes: column_types,
        rows: Array2SerializeWrapper(rows),
    };

    Ok(warp::reply::json(&response))
}

fn first_query_rows(v: Vec<SimpleQueryMessage>) -> Vec<SimpleQueryRow> {
    v.into_iter()
        .map(|msg| if let SimpleQueryMessage::Row(row) = msg { Some(row) } else { None })
        .while_some()
        .collect_vec()
}

fn handle_errors(x: Result<impl Reply, RequestError>) -> Result<warp::reply::Response, Infallible> {
    match x {
        Ok(x) => Ok(x.into_response()),
        Err(e) => {
            let code;
            let message: Cow<_>;

            match e {
                RequestError::InvalidInput(_) => {
                    code = StatusCode::BAD_REQUEST;
                    message = "invalid query string".into();
                }
                RequestError::SqlError(db_error) => {

                    code = StatusCode::BAD_REQUEST;
                    message = format!("SQL error: {}", db_error.message()).into();
                }
                RequestError::ServerError(e) => {
                    println!("error: {:?}", e);

                    code = StatusCode::INTERNAL_SERVER_ERROR;
                    message = "internal server error".into();
                }
            }

            Ok(util::error_response(code, &message))
        }
    }
}

impl From<tokio_postgres::Error> for RequestError {
    fn from(e: tokio_postgres::Error) -> Self {
        let msg = format!("{}", e);

        if let Some(source) = e.into_source() {
            return match source.downcast::<tokio_postgres::error::DbError>() {
                Ok(e) => RequestError::SqlError(*e),
                Err(e) => RequestError::ServerError(format_err!(e)),
            };
        }

        RequestError::ServerError(format_err!(msg))
    }
}
