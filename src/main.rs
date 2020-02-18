mod raw_query;

use anyhow::{Context, Error};
use futures::prelude::*;
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
        let api_help = warp::path("api").and(warp::path::end()).map(|| "This is the API endpoint.");
        let raw_query = warp::path!("api" / "raw-query").and(warp::query()).and_then(
            move |params: raw_query::Params| {
                let pool = pool.clone();
                async move {
                    raw_query::run(&pool, &params.query)
                        .map_err(|e| {
                            println!("error: {:?}", e);
                            warp::reject::custom(ServerError)
                        })
                        .await
                }
            },
        );

        warp::get().and(static_files.or(api_help).or(raw_query))
    };

    let fut = warp::serve(routes).run(([0, 0, 0, 0], listen_port));

    println!("Listening on port {}", listen_port);
    fut.await;

    Ok(())
}

#[derive(Debug)]
struct ServerError;

impl Reject for ServerError {}
