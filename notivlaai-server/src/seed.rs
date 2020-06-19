extern crate diesel;

use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};

use notivlaai_lib::db::{NewCustomer, NewOrder, NewVlaai, NewVlaaiToOrder};
use notivlaai_lib::schema;

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
fn insert_order(
    conn: &SqliteConnection,
    in_transit: bool,
    order_number: Option<i32>,
    name: &str,
    vlaaien: &[&str],
) {
    let client = schema::customer::table
        .filter(schema::customer::name.eq(name))
        .first::<notivlaai_lib::db::Customer>(conn)
        .expect("Could not find customer");

    diesel::insert_into(schema::order::table)
        .values(NewOrder {
            customer_id: client.id,
            in_transit,
            picked_up: false,
            order_number,
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
    let conn = notivlaai_lib::db::establish_connection(false);

    // Insert vlaaien
    insert_vlaai(&conn, "Abrikoos");
    insert_vlaai(&conn, "HalfHalf");
    insert_vlaai(&conn, "Kers");
    insert_vlaai(&conn, "Appel");
    insert_vlaai(&conn, "Kruimelpudding");

    // Insert some customers
    insert_customer(&conn, "Peter Bergmans", "peter@peter.nl");
    insert_customer(&conn, "Piet Pokerface", "pokeren@pokerface.nl");

    insert_order(&conn, false, None, "Peter Bergmans", &["Abrikoos", "Kers"]);
    insert_order(
        &conn,
        true,
        Some(1),
        "Piet Pokerface",
        &["Abrikoos", "Kers"],
    );
}
