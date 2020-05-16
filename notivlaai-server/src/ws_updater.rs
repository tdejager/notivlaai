use futures_util::sink::SinkExt;
use futures_util::{stream::TryStreamExt, StreamExt};
use serde::Serialize;
use std::thread;
use thread::JoinHandle;
use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;

pub struct WsUpdater {
    port: u32,
}

impl WsUpdater {
    pub fn new(port: u32) -> WsUpdater {
        WsUpdater { port }
    }

    pub fn start(self) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut runtime = tokio::runtime::Runtime::new().expect("Could not create runtime");
            runtime.block_on(async { start_server(self.port).await });
        })
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
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (mut outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|_msg| {
        // println!(
        //     "Received a message from {}: {}",
        //     addr,
        //     msg.to_text().unwrap()
        // );
        futures_util::future::ready(Ok(()))
    });
    let send_message = async {
        let mut order_id = 0;
        loop {
            let json = serde_json::to_string(&NotifyOrder {
                client_name: "Tim de Jager".into(),
                id: order_id,
                rows: vec![OrderRow {
                    vlaai: "Abrikoos".into(),
                    amount: 3,
                }],
            })
            .expect("Could not jsonify data");
            order_id += 1;

            outgoing
                .send(Message::text(json))
                .await
                .expect("Could not send message");

            let delay = tokio::time::delay_for(tokio::time::Duration::from_millis(1000));
            delay.await;
        }
    };

    tokio::select! {
        _ = send_message => {}
        _ = broadcast_incoming => {}
    }
}

async fn start_server(port: u32) {
    let addr = format!("localhost:{}", port);
    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let mut listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr));
    }
}
