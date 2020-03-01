use crate::ServerError;
use anyhow::{format_err, Error};
use deadpool_postgres::Pool;
use futures::prelude::*;
use indoc::indoc;
use itertools::Itertools;
use ndarray::Array2;
use serde::{ser::SerializeSeq, Deserialize, Serialize, Serializer};
use serde_json as json;
use tokio_postgres::{SimpleQueryMessage, SimpleQueryRow};
use warp::{Filter, Rejection};

#[derive(Debug, Deserialize)]
pub struct Params {
    pub query: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
struct Response<'a, 'b> {
    pub columnNames: Vec<&'a str>,
    pub columnTypes: Vec<&'b str>,
    pub rows: Array2SerializeWrapper<json::Value>,
}

#[derive(Debug)]
struct Array2SerializeWrapper<T>(Array2<T>);

impl<T> Serialize for Array2SerializeWrapper<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.nrows()))?;
        for row in self.0.genrows() {
            let row = row.as_slice().unwrap();
            seq.serialize_element(row)?;
        }

        seq.end()
    }
}

pub fn run(pool: Pool) -> impl Filter<Extract = (warp::reply::Json,), Error = Rejection> + Clone {
    warp::query().and_then(move |params: Params| {
        let pool = pool.clone();
        async move {
            inner_run(&pool, &params.query)
                .map_err(|e| {
                    println!("error: {:?}", e);
                    warp::reject::custom(ServerError)
                })
                .await
        }
    })
}

async fn inner_run(pool: &Pool, query: &str) -> Result<warp::reply::Json, Error> {
    let mut setup_query = indoc!(
        "
        BEGIN;
        CREATE TEMP TABLE tmp
            ON COMMIT DROP
        AS
        "
    )
    .to_owned();

    let decoded_query = percent_encoding::percent_decode_str(query).decode_utf8()?;
    setup_query.push_str(&decoded_query);
    let db = pool.get().await?;
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
        FROM tmp;
        COMMIT"
    );

    let rows = first_query_rows(db.simple_query(row_query).await?);
    let row_count = rows.len();
    let rows: Result<Vec<_>, Error> = rows
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
                    "integer" | "smallint" | "bigint" => json::Value::from(x.parse::<f64>()?),
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
