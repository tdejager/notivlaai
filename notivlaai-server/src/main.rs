#[macro_use]
extern crate diesel;

extern crate pretty_env_logger;

mod db;
mod schema;
pub mod status_updater;
mod ws_updater;
use serde::{Deserialize, Serialize};
use status_updater::OrderStatusUpdater;
use status_updater::{DBBackend, UpdateOrder};
use tokio::sync::mpsc::Sender;
use warp::http::StatusCode;
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
#[serde(rename_all = "camelCase")]
enum UpdateResponse {
    OK,
}

/// Updating an order
async fn order_retrieved(
    id: u32,
    mut sender: Sender<UpdateOrder>,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    log::info!("GET order_retrieved");

    // Try to send a message to the status updater that the order has been retrieved
    if let Err(_) = sender.send(UpdateOrder::OrderRetrieved(id)).await {
        Ok(warp::reply::with_status(
            warp::reply::json(&UpdateResponse::OK),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        // The request has been completed succesfully
        Ok(warp::reply::with_status(
            warp::reply::json(&"".to_string()),
            StatusCode::OK,
        ))
    }
}

/// GET /order/retrieved/id
fn update_filter(
    sender: Sender<UpdateOrder>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("order" / "retrieved" / u32)
        .and(with_sender(sender))
        .and_then(order_retrieved)
}

async fn warp_main(sender: Sender<UpdateOrder>) {
    let static_files = warp::fs::dir("static");

    warp::serve(
        update_filter(sender)
            .or(static_files)
            .or(warp::path("search").and(warp::fs::file("./static/index.html"))),
    )
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

    // This handles the updating of orders when this is requested by the clien
    let order_status_updater = OrderStatusUpdater::<DBBackend>::new(receiver);
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

#[cfg(test)]
mod tests {
    use crate::status_updater::{OrderPublish, OrderStatusUpdater, TestBackend};
    use warp::http::StatusCode;
    use warp::test::request;

    #[tokio::test]
    async fn test_api() {
        let (sender, receiver) = tokio::sync::mpsc::channel(100);
        let order_status_updater = OrderStatusUpdater::<TestBackend>::new(receiver);
        let update = super::update_filter(sender);

        // Get a subscriber and a runner
        let (subscriber, runner) = order_status_updater.order_mutator();

        // Process order updates
        tokio::spawn(async { runner.run().await });
        let mut sub = subscriber.subscribe();

        // We should be able to send a request
        let resp = request()
            .method("GET")
            .path("/order/retrieved/1")
            .reply(&update)
            .await;

        // The request should return an OK
        assert_eq!(resp.status(), StatusCode::OK);

        // And the subscriber should receieve the updated message
        let message = sub.recv().await;
        // That the order should be removed from the screen
        assert_eq!(message.unwrap(), OrderPublish::RemoveOrder(1));
    }
}
