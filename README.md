# Genetics Database

A web server providing an API to query a genetics database.

### Building and Running

##### Prerequisites:

- A recent version of Rust. To install Rust, follow the directions at [rustup.rs](https://rustup.rs/) (you can go with the default options in the installer).
- Access to a database set up for this project (setup not covered here).

##### Steps:

1. Set the `DATABASE_URL` environment variable based on the database the server will connect to:

    **Mac and Linux:** `export DATABASE_URL='postgres://<user>:<pass>@<host>/<db>'`

    **Windows cmd.exe:** `set DATABASE_URL=postgres://<user>:<pass>@<host>/<db>`

    **Windows PowerShell:** `$env:DATABASE_URL='postgres://<user>:<pass>@<host>/<db>'`

2. To run the server, execute `cargo run` in this directory (or use `cargo run --release` instead for an optimized build). Once running, the API will be available at http://localhost:3030/api. You can change the port by setting the `PORT` environment variable before running.
