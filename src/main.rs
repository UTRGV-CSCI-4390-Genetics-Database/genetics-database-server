use anyhow::{Context, Error};
use deadpool_postgres::Pool;
use futures::prelude::*;
use tokio_postgres::SimpleQueryMessage;
use warp::{reject::Reject, Filter};

const DEFAULT_LISTEN_PORT: u16 = 3030;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let listen_port = std::env::var("PORT")
        .ok()
        .and_then(|var| var.trim().parse::<u16>().ok())
        .unwrap_or(DEFAULT_LISTEN_PORT);

    let pool = {
        let database_url = std::env::var("DATABASE_URL")
            .context("error reading environment variable DATABASE_URL")?;
        let config =
            database_url.parse().context("the environment variable DATABASE_URL is invalid")?;
        let manager = deadpool_postgres::Manager::new(config, tokio_postgres::NoTls);
        deadpool_postgres::Pool::new(manager, 5)
    };

    let routes = {
        let static_files = warp::fs::dir("www");
        let api = warp::path("api");
        let api_help = api.and(warp::path::end()).map(|| "This is the API endpoint.");
        let raw_query = warp::path!("api" / "rawQuery" / String).and_then(move |query: String| {
            let pool = pool.clone();
            async move {
                raw_query(&pool, &query)
                    .map_err(|e| {
                        println!("error: {:?}", e);
                        warp::reject::custom(ServerError)
                    })
                    .await
            }
        });

        warp::get().and(static_files.or(api_help).or(raw_query))
    };

    let fut = warp::serve(routes).run(([0, 0, 0, 0], listen_port));

    println!("Listening on port {}", listen_port);
    fut.await;

    Ok(())
}

#[derive(Debug, Copy, Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct ServerError;

impl Reject for ServerError {}

async fn raw_query(pool: &Pool, query: &str) -> Result<String, Error> {
    let db = pool.get().await?;
    let query_result =
        db.simple_query(&percent_encoding::percent_decode_str(query).decode_utf8()?).await?;
    let mut response = String::new();
    for msg in query_result.into_iter() {
        if let SimpleQueryMessage::Row(row) = msg {
            for i in 0..row.len() {
                response.push_str(&format!("{} ", row.get(i).unwrap_or("NULL")));
            }
            response.push('\n');
        }
    }

    Ok(response)
}
