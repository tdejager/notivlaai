use crate::schema::*;
use connection::SimpleConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use diesel::*;
use lazy_static::lazy_static;
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

#[derive(Associations, Identifiable, Queryable, Copy, Clone)]
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

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OrderRow {
    pub vlaai: String,
    pub amount: u32,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PendingOrder {
    pub id: u32,
    pub customer_name: String,
    pub rows: Vec<OrderRow>,
}

#[derive(Debug)]
pub struct ConnectionOptions {
    pub enable_foreign_keys: bool,
    pub busy_timeout: Option<std::time::Duration>,
}

impl ConnectionOptions {
    pub fn apply(&self, conn: &SqliteConnection) -> QueryResult<()> {
        if self.enable_foreign_keys {
            conn.batch_execute("PRAGMA foreign_keys = ON;")?;
        }
        if let Some(duration) = self.busy_timeout {
            conn.batch_execute(&format!("PRAGMA busy_timeout = {};", duration.as_millis()))?;
        }
        Ok(())
    }
}

impl Default for ConnectionOptions {
    fn default() -> Self {
        Self {
            enable_foreign_keys: true,
            busy_timeout: Some(std::time::Duration::from_secs(10)),
        }
    }
}

impl diesel::r2d2::CustomizeConnection<SqliteConnection, diesel::r2d2::Error>
    for ConnectionOptions
{
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        self.apply(conn).map_err(diesel::r2d2::Error::QueryError)
    }
}

pub type PooledConnection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>>;
lazy_static! {
    /// Create pool singleton
    static ref POOL: Pool<diesel::r2d2::ConnectionManager<SqliteConnection>> = {
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Pool::builder()
            .connection_customizer(Box::new(ConnectionOptions::default()))
            .build(ConnectionManager::<SqliteConnection>::new(database_url))
            .unwrap()
    };
}

/// Create a connection from the connection pool
pub fn establish_connection() -> PooledConnection {
    dotenv::dotenv().ok();

    POOL.get().expect("Could not get connection")
}

/// Get the name of a vlaai for a specific id
pub fn get_vlaai_name(
    conn: &SqliteConnection,
    vlaai_id: i32,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let vlaai_name: String = vlaai::table
        .find(vlaai_id)
        .select(vlaai::name)
        .get_result(conn)?;
    Ok(vlaai_name)
}

#[allow(dead_code)]
/// Retrieve all pending orders
pub fn all_pending_orders(
    conn: &SqliteConnection,
) -> Result<Vec<PendingOrder>, Box<dyn std::error::Error + Send + Sync>> {
    // Get all orders in transit
    let orders: Vec<Order> = order::table
        .filter(order::in_transit.eq(true).and(order::picked_up.eq(false)))
        .load(conn)?;

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
            pending_order.rows.push(OrderRow {
                vlaai: get_vlaai_name(conn, order_row.vlaai_id)?,
                amount: order_row.amount as u32,
            });
        }
        pending_orders.push(pending_order);
    }

    Ok(pending_orders)
}

#[allow(dead_code)]
/// Convert an existing order to a pending one
pub fn to_pending(
    conn: &SqliteConnection,
    order: Order,
) -> Result<PendingOrder, Box<dyn std::error::Error + Send + Sync>> {
    let customer = customer::table
        .find(order.customer_id)
        .get_result::<Customer>(conn)?;

    let order_rows = vlaai_to_order::table
        .filter(vlaai_to_order::order_id.eq(order.id))
        .load::<VlaaiToOrder>(conn)?
        .into_iter()
        .map(|v| {
            get_vlaai_name(conn, v.vlaai_id).map(|name| OrderRow {
                vlaai: name,
                amount: v.amount as u32,
            })
        })
        .collect::<Result<Vec<_>, _>>();

    Ok(PendingOrder {
        id: order.id as u32,
        customer_name: format!("{} {}", customer.first_name, customer.last_name),
        rows: order_rows?,
    })
}

#[allow(dead_code)]
pub fn customer_with_name<T: AsRef<str>>(
    conn: &SqliteConnection,
    name: T,
) -> Result<Vec<Customer>, Box<dyn std::error::Error>> {
    Ok(customer::table
        .filter(
            customer::first_name
                .like(name.as_ref())
                .or(customer::last_name.like(name.as_ref())),
        )
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

#[allow(dead_code)]
pub fn update_order_in_transit(
    conn: &SqliteConnection,
    order_id: i32,
) -> Result<Order, Box<dyn std::error::Error + Send + Sync>> {
    diesel::update(order::table.find(order_id))
        .set((order::in_transit.eq(true), order::picked_up.eq(false)))
        .execute(conn)?;
    Ok(order::table.find(order_id).get_result(conn)?)
}

#[allow(dead_code)]
pub fn update_order_retrieved(
    conn: &SqliteConnection,
    order_id: i32,
) -> std::result::Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    Ok(diesel::update(order::table.find(order_id))
        .set((order::in_transit.eq(false), order::picked_up.eq(true)))
        .execute(conn)?)
}

#[allow(dead_code)]
pub fn update_order_new(
    conn: &SqliteConnection,
    order_id: i32,
) -> std::result::Result<usize, Box<dyn std::error::Error>> {
    Ok(diesel::update(order::table.find(order_id))
        .set((order::in_transit.eq(false), order::picked_up.eq(false)))
        .execute(conn)?)
}

#[cfg(test)]
mod test {

    use diesel::*;
    #[test]
    pub fn get_client_with_name() {
        let conn = super::establish_connection();
        let results =
            super::customer_with_name(&conn, "%pie%").expect("Could not find customer with name");
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
        assert!(pending_orders.len() > 0);
    }

    #[test]
    pub fn updating_order() {
        let conn = super::establish_connection();
        // Change to new
        assert!(super::update_order_new(&conn, 1).expect("Could not update order to be new") > 0);
        // Set to retrieved
        assert!(
            super::update_order_retrieved(&conn, 1).expect("Could not update order to retrieved")
                > 0
        );

        let pending_orders =
            super::all_pending_orders(&conn).expect("Could not retreive pending orders");
        println!("{:?}", pending_orders);
        assert_eq!(pending_orders.len(), 1);

        // Set to status as in seed and return
        let order = super::update_order_in_transit(&conn, 1);
        assert!(order.is_ok());

        if let Ok(order) = order {
            assert_eq!(order.picked_up, false);
            assert_eq!(order.in_transit, true);
        }
    }

    #[test]
    pub fn pending() {
        let conn = super::establish_connection();
        assert!(super::to_pending(
            &conn,
            super::order::table
                .find(1)
                .get_result(&conn)
                .expect("Could not find order")
        )
        .is_ok())
    }
}
