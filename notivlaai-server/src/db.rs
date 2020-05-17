use crate::schema::*;
use diesel::*;
use diesel::{Connection, SqliteConnection};
use serde::Serialize;

#[derive(Associations, Identifiable, Queryable)]
#[table_name = "customer"]
pub struct Customer {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Associations, Identifiable, Queryable)]
#[table_name = "vlaai"]
pub struct Vlaai {
    pub id: i32,
    pub name: String,
}

#[derive(Associations, Identifiable, Queryable)]
#[belongs_to(Customer)]
#[table_name = "order"]
pub struct Order {
    pub id: i32,
    pub customer_id: i32,
    pub in_transit: bool,
    pub picked_up: bool,
}

#[derive(Associations, Identifiable, Queryable)]
#[belongs_to(Order)]
#[table_name = "vlaai_to_order"]
pub struct VlaaiToOrder {
    pub id: i32,
    pub order_id: i32,
    pub vlaai_id: i32,
    pub amount: i32,
}

#[derive(Insertable)]
#[table_name = "customer"]
pub struct NewCustomer<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
}

#[derive(Insertable)]
#[table_name = "vlaai"]
pub struct NewVlaai<'a> {
    pub name: &'a str,
}

#[derive(Insertable)]
#[table_name = "order"]
pub struct NewOrder {
    pub customer_id: i32,
    pub in_transit: bool,
    pub picked_up: bool,
}

#[derive(Insertable)]
#[table_name = "vlaai_to_order"]
pub struct NewVlaaiToOrder {
    pub order_id: i32,
    pub vlaai_id: i32,
    pub amount: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRow {
    pub vlaai: String,
    pub amount: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingOrder {
    pub id: u32,
    pub customer_name: String,
    pub rows: Vec<OrderRow>,
}

pub fn establish_connection() -> SqliteConnection {
    dotenv::dotenv().ok();

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[allow(dead_code)]
/// Retreive all pending orders
pub fn all_pending_orders(
    conn: &SqliteConnection,
) -> Result<Vec<PendingOrder>, Box<dyn std::error::Error>> {
    // Get all orders in transit
    let orders: Vec<Order> = order::table.filter(order::in_transit.eq(true)).load(conn)?;

    let mut pending_orders = Vec::new();
    // Map onto customers and vlaaien
    for order in orders {
        // Find customer
        let customer: Customer = customer::table.find(order.id).get_result(conn)?;
        // Find vlaaien
        let order_rows = vlaai_to_order::table
            .filter(vlaai_to_order::order_id.eq(order.id))
            .load::<VlaaiToOrder>(conn)?;

        // Create the pending order
        let mut pending_order = PendingOrder {
            id: order.id as u32,
            customer_name: format!("{} {}", customer.first_name, customer.last_name),
            rows: Default::default(),
        };

        // Fill in the order rows
        for order_row in order_rows {
            let vlaai_name: String = vlaai::table
                .find(order_row.vlaai_id)
                .select(vlaai::name)
                .get_result(conn)?;
            pending_order.rows.push(OrderRow {
                vlaai: vlaai_name,
                amount: order_row.amount as u32,
            });
        }
        pending_orders.push(pending_order);
    }

    Ok(pending_orders)
}

#[allow(dead_code)]
pub fn customer_with_name<T: AsRef<str>>(
    conn: &SqliteConnection,
    name: T,
) -> Result<Vec<Customer>, Box<dyn std::error::Error>> {
    Ok(customer::table
        .filter(customer::first_name.like(name.as_ref()))
        .load(conn)?)
}

#[allow(dead_code)]
pub fn orders_for_customer(
    conn: &SqliteConnection,
    id: i32,
) -> Result<Vec<Order>, Box<dyn std::error::Error>> {
    // Find the customer
    let customer: Customer = customer::table.find(id).get_result(conn)?;
    Ok(Order::belonging_to(&customer).load(conn)?)
}

#[cfg(test)]
mod test {

    #[test]
    pub fn get_client_with_name() {
        let conn = super::establish_connection();
        let results =
            super::customer_with_name(&conn, "pie%").expect("Could not find customer with name");
        assert!(results.len() > 0)
    }

    #[test]
    pub fn order_for_customer() {
        let conn = super::establish_connection();
        let results = super::orders_for_customer(&conn, 1)
            .expect("Could not find orders for customer with this id");
        assert!(results.len() > 0)
    }

    #[test]
    pub fn pending_orders() {
        let conn = super::establish_connection();
        let pending_orders =
            super::all_pending_orders(&conn).expect("Could not retreive pending orders");
        assert!(pending_orders.len() > 0)
    }
}
