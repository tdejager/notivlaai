#[macro_use]
extern crate diesel;

extern crate pretty_env_logger;

pub mod db;
mod schema;
pub mod status_updater;
mod ws_updater;
use warp::Filter;

/// Use the environment variable for static files, otherwise assume it is the project dir
pub fn static_file_location() -> String {
    dotenv::var("STATIC_FILES")
        .unwrap_or_else(|_| concat!(env!("CARGO_MANIFEST_DIR"), "/static").to_string())
}

async fn warp_main() {
    // GET /hi
    let hi = warp::path("hi").map(|| "Hello, World!");
    let log = warp::log("static");
    let static_files = warp::fs::dir("static").with(log);

    warp::serve(hi.or(static_files))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn main() {
    // Load environment file
    dotenv::dotenv().ok();

    // Initialize the logger
    pretty_env_logger::init();

    let mut runtime = tokio::runtime::Runtime::new().expect("Could not construct runtime");
    let handler = ws_updater::WsUpdater::new(9001);

    // Tokio runtime
    runtime.block_on(async {
        // Run the web-client
        tokio::spawn(async { warp_main().await });

        // Run the websocket handler
        handler.start().await
    });
}
