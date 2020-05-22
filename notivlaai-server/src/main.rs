#[macro_use]
extern crate diesel;

extern crate pretty_env_logger;

mod db;
mod schema;
pub mod status_updater;
mod ws_updater;
use serde::{Deserialize, Serialize};
use status_updater::OrderStatusUpdater;
use status_updater::UpdateOrder;
use tokio::sync::mpsc::Sender;
use warp::Filter;

/// Use the environment variable for static files, otherwise assume it is the project dir
pub fn static_file_location() -> String {
    dotenv::var("STATIC_FILES")
        .unwrap_or_else(|_| concat!(env!("CARGO_MANIFEST_DIR"), "/static").to_string())
}

/// Couples a sender to add to a filter
fn with_sender(
    sender: Sender<UpdateOrder>,
) -> impl Filter<Extract = (Sender<UpdateOrder>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || sender.clone())
}

#[derive(Serialize, Deserialize)]
enum UpdateResponse {
    OK,
}

/// Updating an order
fn update_order(_sender: Sender<UpdateOrder>) -> warp::reply::Json {
    warp::reply::json(&UpdateResponse::OK)
}

async fn warp_main(sender: Sender<UpdateOrder>) {
    // GET /hi
    let update_order = warp::path("updater_order")
        .and(with_sender(sender))
        .map(update_order);
    let log = warp::log("static");
    let static_files = warp::fs::dir("static").with(log);

    warp::serve(update_order.or(static_files))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn main() {
    // Load environment file
    dotenv::dotenv().ok();

    // Initialize the logger
    pretty_env_logger::init();

    let mut runtime = tokio::runtime::Runtime::new().expect("Could not construct runtime");
    let (sender, receiver) = tokio::sync::mpsc::channel(100);
    let order_status_updater = OrderStatusUpdater::new(receiver);
    let handler = ws_updater::WsUpdater::new(9001);

    // Tokio runtime
    runtime.block_on(async {
        let (subscriber, runner) = order_status_updater.order_mutator();
        // Run the web-client
        tokio::spawn(async { warp_main(sender).await });

        // Run the websocket handler
        handler.start(subscriber, runner).await
    });
}
