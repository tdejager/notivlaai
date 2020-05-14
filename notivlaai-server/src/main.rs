#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
use rocket::{
    config::{Environment, Value},
    Config,
};
use rocket_contrib::serve::StaticFiles;
use serde::Serialize;
use std::collections::HashMap;
use std::net::TcpListener;
use tungstenite::server::accept;
use tungstenite::Message;

mod db;
mod schema;

#[database("notivlaai")]
struct NotivlaaiDb(diesel::SqliteConnection);

#[get("/")]
fn index(_conn: NotivlaaiDb) -> &'static str {
    "Hello, world!"
}

/// Create rocket config from environment variables
pub fn from_env() -> Config {
    let environment = Environment::active().expect("No environment found");

    let port = dotenv::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    let database_url =
        dotenv::var("DATABASE_URL").expect("No DATABASE_URL environment variable found");
    database_config.insert("url", Value::from(database_url));
    databases.insert("notivlaai", Value::from(database_config));

    Config::build(environment)
        .environment(environment)
        .port(port)
        .extra("databases", databases)
        .finalize()
        .unwrap()
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

fn main() {
    // Load environment file
    dotenv::dotenv().ok();

    std::thread::spawn(|| {
        let server = TcpListener::bind("127.0.0.1:9001").unwrap();
        for stream in server.incoming() {
            let mut websocket = accept(stream.unwrap()).unwrap();
            let mut order_id = 0u32;
            loop {
                let json = serde_json::to_string(&NotifyOrder {
                    client_name: "Tim de Jager".into(),
                    id: order_id,
                    rows: vec![OrderRow{vlaai: "Abrikoos".into(), amount: 3 }],
                }).expect("Could not jsonify data");

                // Try to receive a message
                let msg = websocket.write_message(Message::text(json));
                order_id += 1;

                match msg {
                    Ok(_) => {
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                    }
                    // Break if the connection is closed
                    Err(tungstenite::Error::ConnectionClosed) => break,
                    Err(_) => panic!("An ws error occured"),
                }
            }
        }
    });

    // Custom config
    rocket::custom(from_env())
        // Attach the database
        .attach(NotivlaaiDb::fairing())
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .mount("/data", routes![index])
        .launch();
}
