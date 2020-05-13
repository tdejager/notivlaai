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
