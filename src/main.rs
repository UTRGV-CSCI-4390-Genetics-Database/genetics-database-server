mod raw_query;
mod util;

use anyhow::{Context, Error};
use deadpool_postgres::Pool;
use warp::{Filter, Reply};

const DEFAULT_LISTEN_PORT: u16 = 3030;
const MAX_DATABASE_CONNECTIONS: usize = 5;

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
        deadpool_postgres::Pool::new(manager, MAX_DATABASE_CONNECTIONS)
    };

    let fut = warp::serve(routes(pool)).run(([0, 0, 0, 0], listen_port));

    println!("Listening on port {}", listen_port);
    fut.await;

    Ok(())
}

fn routes(pool: Pool) -> impl Filter<Extract = impl Reply> + Clone + Send + Sync + 'static {
    let static_files = warp::fs::dir("www");
    let api_help = warp::path("api").and(warp::path::end()).map(|| "This is the API endpoint.");
    let raw_query = warp::path!("api" / "raw-query").and(raw_query::create(pool.clone()));

    let cors = warp::cors().allow_any_origin().build();
    warp::get().and(static_files.or(api_help).or(raw_query)).with(cors)
}
