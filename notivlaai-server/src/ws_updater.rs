use crate::db;
use crate::status_updater::{DBBackend, OrderPublish, OrderRunner, OrderSubscriber};
use futures_util::sink::SinkExt;
use futures_util::{stream::TryStreamExt, StreamExt};
use log::info;
use serde::Serialize;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast::{Receiver, RecvError},
};
use tungstenite::protocol::Message;

/// Update the order screen using websockets
pub struct WsUpdater {
    port: u32,
}

impl WsUpdater {
    pub fn new(port: u32) -> WsUpdater {
        WsUpdater { port }
    }

    pub async fn start(self, subscriber: OrderSubscriber, runner: OrderRunner<DBBackend>) {
        start_server(self.port, subscriber, runner).await;
    }
}

/// This is an enum that is sent to the typescript side
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderNotification {
    /// Initialize with all pending orders
    Initialize(Vec<db::PendingOrder>),
    /// Add an order
    AddOrder(db::PendingOrder),
    /// Remove an order
    RemoveOrder(u32),
}

impl From<OrderPublish> for OrderNotification {
    fn from(pubish: OrderPublish) -> Self {
        match pubish {
            OrderPublish::AddOrder(p) => OrderNotification::AddOrder(p),
            OrderPublish::RemoveOrder(idx) => OrderNotification::RemoveOrder(idx),
        }
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: std::net::SocketAddr,
    mut receiver: Receiver<OrderPublish>,
) {
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

    let pending =
        crate::db::all_pending_orders(&conn).expect("Could not get pending orders from database");
    let json = serde_json::to_string(&OrderNotification::Initialize(pending))
        .expect("Could not serialze pending orders to json");
    outgoing
        .send(Message::text(json))
        .await
        .expect("Could not send message");

    let send_message = async move {
        // Receive order updates
        loop {
            let message = receiver.recv().await;
            // If we are closed break out of this
            if let Err(RecvError::Closed) = message {
                break;
            }
            // Otherwise just go on
            if let Ok(value) = message {
                outgoing
                    .send(Message::text(
                        serde_json::to_string(&OrderNotification::from(value))
                            .expect("Could not convert update to json"),
                    ))
                    .await
                    .expect("Could not send update");
            };
        }
    };

    tokio::select! {
        _ = send_message => {}
        _ = broadcast_incoming => {}
    }
}

async fn start_server(port: u32, subscriber: OrderSubscriber, runner: OrderRunner<DBBackend>) {
    let addr = format!("localhost:{}", port);

    // Wait for new updates
    tokio::spawn(async move { runner.run().await });

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let mut listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);
    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        let receiver = subscriber.subscribe();
        tokio::spawn(handle_connection(stream, addr, receiver));
    }
}
