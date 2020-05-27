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
/// Couples a sender to add to a filter
fn with_conn(
) -> impl Filter<Extract = (db::PooledConnection,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db::establish_connection())
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
            warp::reply::json(&"".to_string()),
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

/// Updating an order
async fn order_in_transit(
    id: u32,
    mut sender: Sender<UpdateOrder>,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    log::info!("GET order_in_transit");

    // Try to send a message to the status updater that the order has been retrieved
    if let Err(_) = sender.send(UpdateOrder::OrderInTransit(id)).await {
        Ok(warp::reply::with_status(
            warp::reply::json(&"".to_string()),
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

fn find_client(name: String, conn: db::PooledConnection) -> impl warp::Reply {
    let customer =
        db::customer_with_name(&conn, format!("%{}%", name)).expect("Could not retrieve customer");
    let names: Vec<(i32, String)> = customer
        .iter()
        .map(|c| (c.id, format!("{} {}", c.first_name, c.last_name)))
        .collect();
    Ok(warp::reply::json(&names))
}

fn find_order(id: u32, conn: db::PooledConnection) -> impl warp::Reply {
    let orders =
        db::orders_for_customer(&conn, id as i32).expect("Could not retrieve orders for customer");
    let orders: Vec<db::PendingOrder> = orders
        .into_iter()
        .filter_map(|o| db::to_pending(&conn, o).ok())
        .collect();
    Ok(warp::reply::json(&orders))
}

/// GET /client/find/:name
fn find_client_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("customer" / "find" / String)
        .and(with_conn())
        .map(find_client)
}

/// GET /order/find/:customer_id
fn find_order_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("order" / "find" / u32)
        .and(with_conn())
        .map(find_order)
}

/// GET /order/retrieved/:order_id
fn update_filter(
    sender: Sender<UpdateOrder>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("order" / "retrieved" / u32)
        .and(with_sender(sender))
        .and_then(order_retrieved)
}

/// GET /order/in_transit/:order_id
fn in_transit_filter(
    sender: Sender<UpdateOrder>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("order" / "in_transit" / u32)
        .and(with_sender(sender))
        .and_then(order_in_transit)
}

async fn warp_main(sender: Sender<UpdateOrder>) {
    let static_files = warp::fs::dir("static");

    warp::serve(
        update_filter(sender.clone())
            .or(find_client_filter())
            .or(find_order_filter())
            .or(in_transit_filter(sender))
            .or(warp::path("search").and(warp::fs::file("./static/index.html")))
            .or(static_files),
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

    #[tokio::test]
    async fn test_api_in_transit() {
        let (sender, receiver) = tokio::sync::mpsc::channel(100);
        let order_status_updater = OrderStatusUpdater::<TestBackend>::new(receiver);
        let update = super::in_transit_filter(sender);

        // Get a subscriber and a runner
        let (subscriber, runner) = order_status_updater.order_mutator();

        // Process order updates
        tokio::spawn(async { runner.run().await });
        let mut sub = subscriber.subscribe();

        // We should be able to send a request
        let resp = request()
            .method("GET")
            .path("/order/in_transit/1")
            .reply(&update)
            .await;

        // The request should return an OK
        assert_eq!(resp.status(), StatusCode::OK);

        // And the subscriber should receieve the updated message
        let _message = sub.recv().await;
        // That the order should be removed from the screen
        //assert_eq!(message.unwrap(), OrderPublish::AddOrder(1));
    }
    #[tokio::test]
    async fn test_get_client() {
        let client = super::find_client_filter();

        let resp = request()
            .method("GET")
            .path("/customer/find/pie")
            .reply(&client)
            .await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_order() {
        let client = super::find_order_filter();

        let resp = request()
            .method("GET")
            .path("/order/find/1")
            .reply(&client)
            .await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
