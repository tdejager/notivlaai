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

pub mod db;
mod schema;
pub mod status_updater;
mod ws_updater;

//#[database("notivlaai")]
//struct NotivlaaiDb(diesel::SqliteConnection);

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

/// Use the environment variable for static files, otherwise assume it is the project dir
pub fn static_file_location() -> String {
    dotenv::var("STATIC_FILES")
        .unwrap_or_else(|_| concat!(env!("CARGO_MANIFEST_DIR"), "/static").to_string())
}

#[post("/retrieved/<id>")]
fn order_delivered(id: u32) {
    let connection = db::establish_connection();
    db::update_order_retrieved(&connection, id as i32).expect("Could not update order");
}

fn main() {
    // Load environment file
    dotenv::dotenv().ok();

    let handler = ws_updater::WsUpdater::new(9001);
    let handle = handler.start();

    // Custom config
    rocket::custom(from_env())
        // Attach the database
        //.attach(NotivlaaiDb::fairing())
        .mount("/", StaticFiles::from(static_file_location()))
        .mount("/orders", routes![order_delivered])
        .launch();

    // should_close.store(true, Ordering::Relaxed);
    handle.join().expect("Cannot join ws thread");
}
