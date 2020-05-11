#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::databases::diesel;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
use rocket_contrib::serve::StaticFiles;

#[database("notivlaai")]
struct NotivlaaiDb(diesel::SqliteConnection);

#[get("/")]
fn index(_conn: NotivlaaiDb) -> &'static str {
    "Hello, world!"
}

//pub fn establish_connection() -> OriginalSqliteConn {
//dotenv::dotenv().ok();

//let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
//OriginalSqliteConn::establish(&database_url)
//.expect(&format!("Error connecting to {}", database_url))
//}

fn main() {
    rocket::ignite()
        .attach(NotivlaaiDb::fairing())
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .mount("/data", routes![index])
        .launch();
}
