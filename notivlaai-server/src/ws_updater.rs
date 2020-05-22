use crate::status_updater::{OrderRunner, OrderSubscriber};
use futures_util::sink::SinkExt;
use futures_util::{stream::TryStreamExt, StreamExt};
use log::info;
use serde::Serialize;
use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;

pub struct WsUpdater {
    port: u32,
}

impl WsUpdater {
    pub fn new(port: u32) -> WsUpdater {
        WsUpdater { port }
    }

    pub async fn start(self, subscriber: OrderSubscriber, runner: OrderRunner) {
        start_server(self.port, subscriber, runner).await;
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct OrderRow {
    pub vlaai: String,
    pub amount: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NotifyOrder {
    pub id: u32,
    pub client_name: String,
    pub rows: Vec<OrderRow>,
}

async fn handle_connection(stream: TcpStream, addr: std::net::SocketAddr) {
    info!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    info!("WebSocket connection established: {}", addr);

    // Create a sqlite connection
    let conn = crate::db::establish_connection();

    let (mut outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|_msg| {
        // println!(
        //     "Received a message from {}: {}",
        //     addr,
        //     msg.to_text().unwrap()
        // );
        futures_util::future::ready(Ok(()))
    });

    // Retrieve all pending orders
    let send_message = async move {
        // Send these over the websocket
        loop {
            let pending = crate::db::all_pending_orders(&conn)
                .expect("Could not get pending orders from database");
            let json =
                serde_json::to_string(&pending).expect("Could not serialze pending orders to json");
            outgoing
                .send(Message::text(json))
                .await
                .expect("Could not send message");
            tokio::time::delay_for(tokio::time::Duration::from_millis(100)).await;
        }
    };

    tokio::select! {
        _ = send_message => {}
        _ = broadcast_incoming => {}
    }
}

async fn start_server(port: u32, subscriber: OrderSubscriber, runner: OrderRunner) {
    let addr = format!("localhost:{}", port);

    // Wait for new updates
    tokio::spawn(async move { runner.run().await });

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let mut listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);
    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        let _receiver = subscriber.subscribe();
        tokio::spawn(handle_connection(stream, addr));
    }
}
