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

use std::collections::HashMap;

mod db;
mod schema;
mod ws_updater;

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

fn main() {
    // Load environment file
    dotenv::dotenv().ok();

    // let should_close = std::sync::Arc::new(AtomicBool::new(false));

    // let should_close_thread = should_close.clone();
    // let handle = std::thread::spawn(move || {
    //     let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    //     for stream in server.incoming() {
    //         // We should close because we are at the end of the program
    //         if should_close_thread.load(Ordering::Relaxed) {
    //             break;
    //         }
    //         println!("Accepting socket connection");
    //         let mut websocket = accept(stream.unwrap()).unwrap();
    //         let mut order_id = 0u32;
    //         loop {
    //             // We should close because we are at the end of the program
    //             if should_close_thread.load(Ordering::Relaxed) {
    //                 break;
    //             }

    //             let json = serde_json::to_string(&NotifyOrder {
    //                 client_name: "Tim de Jager".into(),
    //                 id: order_id,
    //                 rows: vec![OrderRow {
    //                     vlaai: "Abrikoos".into(),
    //                     amount: 3,
    //                 }],
    //             })
    //             .expect("Could not jsonify data");

    //             // Try to receive a message
    //             websocket
    //                 .write_message(Message::text(json))
    //                 .expect("Could not send websocket message");
    //             let msg = websocket.read_message();
    //             order_id += 1;

    //             match msg {
    //                 Ok(_) => {
    //                     println!("Connection is ok!");
    //                     std::thread::sleep(std::time::Duration::from_millis(1000));
    //                 }
    //                 // Break if the connection is closed
    //                 Err(tungstenite::Error::ConnectionClosed) => {
    //                     println!("Connection closed");
    //                     break;
    //                 }
    //                 Err(e) => panic!("An ws error occured: {}", e),
    //             }
    //         }
    //     }
    // });

    let handler = ws_updater::WsUpdater::new(9001);
    let handle = handler.start();

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

    // should_close.store(true, Ordering::Relaxed);
    handle.join().expect("Cannot join ws thread");
}
