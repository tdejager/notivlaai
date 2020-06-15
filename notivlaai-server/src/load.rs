#[macro_use]
extern crate diesel_migrations;

use diesel::prelude::*;
use diesel::SqliteConnection;
use notivlaai_lib::db::{NewCustomer, NewOrder, NewVlaai, NewVlaaiToOrder};

use notivlaai_lib::schema;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CSVRecord {
    name: String,
    abrikoos: Option<i32>,
    kers: Option<i32>,
    halfhalf: Option<i32>,
    rijst: Option<i32>,
    kruimelpudding: Option<i32>,
    appel: Option<i32>,
    email: Option<String>,
    speltak: Option<String>,
}

embed_migrations!();

/// Insert a vlaai into the database
fn insert_vlaai(conn: &SqliteConnection, name: &str) {
    let vlaai = NewVlaai { name };
    diesel::insert_into(schema::vlaai::table)
        .values(vlaai)
        .execute(conn)
        .expect("Could not insert vlaai");
}

/// Insert a customer
fn insert_customer(conn: &SqliteConnection, name: &str, email: &str) {
    let customer = NewCustomer { name, email };
    diesel::insert_into(schema::customer::table)
        .values(customer)
        .execute(conn)
        .expect("Could not insert vlaai");
}

/// Insert an order
fn insert_order(conn: &SqliteConnection, in_transit: bool, name: &str, vlaaien: &[(&str, i32)]) {
    let client = schema::customer::table
        .filter(schema::customer::name.eq(name))
        .first::<notivlaai_lib::db::Customer>(conn)
        .expect("Could not find customer");

    diesel::insert_into(schema::order::table)
        .values(NewOrder {
            customer_id: client.id,
            in_transit,
            picked_up: false,
        })
        .execute(conn)
        .expect("Could not insert order");

    let order_id: i32 = schema::order::table
        .filter(schema::order::customer_id.eq(client.id))
        .select(schema::order::id)
        .first(conn)
        .expect("Could not find order");

    for (vlaai, amount) in vlaaien {
        let vlaai_id: i32 = schema::vlaai::table
            .filter(schema::vlaai::name.eq(vlaai))
            .select(schema::vlaai::id)
            .first(conn)
            .expect("Could not get vlaai");

        diesel::insert_into(schema::vlaai_to_order::table)
            .values(NewVlaaiToOrder {
                order_id,
                vlaai_id,
                amount: *amount,
            })
            .execute(conn)
            .expect("Could not insert vlaai -> order");
    }
}

/// Create a vlaaien record from the CSVRecord
fn record_to_vlaai(record: &CSVRecord) -> Vec<(&str, i32)> {
    let mut slice = Vec::new();

    if let Some(amount) = record.abrikoos {
        slice.push(("Abrikoos", amount));
    }
    if let Some(amount) = record.halfhalf {
        slice.push(("HalfHalf", amount));
    }
    if let Some(amount) = record.kers {
        slice.push(("Kers", amount));
    }
    if let Some(amount) = record.appel {
        slice.push(("Appel", amount));
    }
    if let Some(amount) = record.kruimelpudding {
        slice.push(("Kruimelpudding", amount));
    }
    if let Some(amount) = record.rijst {
        slice.push(("Rijst", amount));
    }

    slice
}

fn main() {
    let conn = notivlaai_lib::db::establish_connection(false);
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
        .expect("Could not run migrations");

    // Insert vlaaien
    insert_vlaai(&conn, "Abrikoos");
    insert_vlaai(&conn, "HalfHalf");
    insert_vlaai(&conn, "Kers");
    insert_vlaai(&conn, "Appel");
    insert_vlaai(&conn, "Rijst");
    insert_vlaai(&conn, "Kruimelpudding");

    let mut rdr = csv::Reader::from_path("./orders.csv").expect("Could not open reader");

    for result in rdr.deserialize() {
        let record: CSVRecord = result.expect("could not decode result");

        // Skip these because they are not orders
        if record.name.is_empty() || record.name.starts_with("Totaal") {
            continue;
        }

        insert_customer(
            &conn,
            &record.name,
            &record.email.clone().unwrap_or("".to_owned()),
        );

        insert_order(&conn, false, &record.name, &record_to_vlaai(&record));

        println!("Inserted {:?}", record);
    }
}
