#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

pub mod db;
mod schema;
pub mod status_updater;
mod ws_updater;

//#[database("notivlaai")]
//struct NotivlaaiDb(diesel::SqliteConnection);

/// Use the environment variable for static files, otherwise assume it is the project dir
pub fn static_file_location() -> String {
    dotenv::var("STATIC_FILES")
        .unwrap_or_else(|_| concat!(env!("CARGO_MANIFEST_DIR"), "/static").to_string())
}

async fn warp_main() {
    warp::serve(warp::fs::dir(static_file_location()))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn main() {
    // Load environment file
    dotenv::dotenv().ok();

    let mut runtime = tokio::runtime::Runtime::new().expect("Could not construct runtime");
    let handler = ws_updater::WsUpdater::new(9001);
    runtime.block_on(async {
        tokio::spawn(async { warp_main().await });
        handler.start().await
    });

    // should_close.store(true, Ordering::Relaxed);
}
