use db::{NewCustomer, NewOrder, NewVlaai, NewVlaaiToOrder};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};

#[macro_use]
extern crate diesel;

mod db;
mod schema;

/// Insert a vlaai into the database
fn insert_vlaai(conn: &SqliteConnection, name: &str) {
    let vlaai = NewVlaai { name };
    diesel::insert_into(schema::vlaai::table)
        .values(vlaai)
        .execute(conn)
        .expect("Could not insert vlaai");
}

/// Insert a customer
fn insert_customer(conn: &SqliteConnection, first_name: &str, last_name: &str, email: &str) {
    let customer = NewCustomer {
        first_name,
        last_name,
        email,
    };
    diesel::insert_into(schema::customer::table)
        .values(customer)
        .execute(conn)
        .expect("Could not insert vlaai");
}

/// Insert an order
fn insert_order(conn: &SqliteConnection, name: &str, vlaaien: &[&str]) {
    let client = schema::customer::table
        .filter(schema::customer::first_name.eq(name))
        .first::<db::Customer>(conn)
        .expect("Could not find customer");

    diesel::insert_into(schema::order::table)
        .values(NewOrder {
            customer_id: client.id,
            in_transit: true,
            picked_up: false,
        })
        .execute(conn)
        .expect("Could not insert order");

    let order_id: i32 = schema::order::table
        .filter(schema::order::customer_id.eq(client.id))
        .select(schema::order::id)
        .first(conn)
        .expect("Could not find order");

    for vlaai in vlaaien {
        let vlaai_id: i32 = schema::vlaai::table
            .filter(schema::vlaai::name.eq(vlaai))
            .select(schema::vlaai::id)
            .first(conn)
            .expect("Could not get vlaai");

        diesel::insert_into(schema::vlaai_to_order::table)
            .values(NewVlaaiToOrder {
                order_id,
                vlaai_id,
                amount: 1,
            })
            .execute(conn)
            .expect("Could not insert vlaai -> order");
    }
}

fn main() {
    let conn = db::establish_connection();

    // Insert vlaaien
    insert_vlaai(&conn, "Abrikoos");
    insert_vlaai(&conn, "HalfHalf");
    insert_vlaai(&conn, "Kers");
    insert_vlaai(&conn, "Appel");
    insert_vlaai(&conn, "Kruimelpudding");

    // Insert some customers
    insert_customer(&conn, "Peter", "Bergmans", "peter@peter.nl");
    insert_customer(&conn, "Piet", "Pokerface", "pokeren@pokerface.nl");

    insert_order(&conn, "Peter", &["Abrikoos", "Kers"]);
    insert_order(&conn, "Piet", &["Abrikoos", "Kers"]);
}
