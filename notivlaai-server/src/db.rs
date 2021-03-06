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
    pub name: String,
    pub email: Option<String>,
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
    pub order_number: Option<i32>,
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
    pub name: &'a str,
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
    pub order_number: Option<i32>,
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
    pub in_transit: bool,
    pub picked_up: bool,
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

/// Apply the connection options here, so that we can use foreign keys and multiple readers
impl diesel::r2d2::CustomizeConnection<SqliteConnection, diesel::r2d2::Error>
    for ConnectionOptions
{
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        self.apply(conn).map_err(diesel::r2d2::Error::QueryError)
    }
}

/// Get the database url depending if we are in production or development
fn get_database_url() -> String {
    if dotenv::var("MODE").unwrap() == "dev" {
        dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set")
    } else {
        dotenv::var("DATABASE_URL_PROD").expect("DATABASE_URL_PROD must be set")
    }
}

/// This is a connction pool
pub type PooledConnection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>>;

// Create the pool singleton here
lazy_static! {
    /// Create pool singleton
    static ref POOL: Pool<diesel::r2d2::ConnectionManager<SqliteConnection>> = {
        let database_url = get_database_url();
        Pool::builder()
            .connection_customizer(Box::new(ConnectionOptions::default()))
            .build(ConnectionManager::<SqliteConnection>::new(database_url))
            .unwrap()
    };
}

/// Create a connection from the connection pool
pub fn establish_connection(is_test: bool) -> PooledConnection {
    dotenv::dotenv().ok();
    if is_test {
        std::env::set_var("MODE", "dev")
    }
    POOL.get().expect("Could not get connection")
}

/// Get the name of a vlaai for a specific id
pub fn get_vlaai_name(conn: &SqliteConnection, vlaai_id: i32) -> anyhow::Result<String> {
    let vlaai_name: String = vlaai::table
        .find(vlaai_id)
        .select(vlaai::name)
        .get_result(conn)?;
    Ok(vlaai_name)
}

/// Retrieve all pending orders
pub fn all_pending_orders(conn: &SqliteConnection) -> anyhow::Result<Vec<PendingOrder>> {
    // Get all orders in transit
    let orders: Vec<Order> = order::table
        .filter(order::in_transit.eq(true).and(order::picked_up.eq(false)))
        .order_by(order::order_number)
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
            picked_up: order.picked_up,
            in_transit: order.in_transit,
            customer_name: customer.name,
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

/// Convert an existing order to a pending one
pub fn to_pending(conn: &SqliteConnection, order: Order) -> anyhow::Result<PendingOrder> {
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
        picked_up: order.picked_up,
        in_transit: order.in_transit,
        customer_name: customer.name,
        rows: order_rows?,
    })
}

#[allow(dead_code)]
pub fn customer_with_name<T: AsRef<str>>(
    conn: &SqliteConnection,
    name: T,
) -> anyhow::Result<Vec<Customer>> {
    Ok(customer::table
        .filter(customer::name.like(name.as_ref()))
        .load(conn)?)
}

pub fn orders_for_customer(conn: &SqliteConnection, id: i32) -> anyhow::Result<Vec<Order>> {
    // Find the customer
    let customer: Customer = customer::table.find(id).get_result(conn)?;
    Ok(Order::belonging_to(&customer).load(conn)?)
}

pub fn update_order_in_transit(
    conn: &SqliteConnection,
    order_id: i32,
    order_number: i32,
) -> anyhow::Result<Order> {
    diesel::update(order::table.find(order_id))
        .set((
            order::in_transit.eq(true),
            order::picked_up.eq(false),
            order::order_number.eq(order_number),
        ))
        .execute(conn)?;
    Ok(order::table.find(order_id).get_result(conn)?)
}

pub fn update_order_retrieved(conn: &SqliteConnection, order_id: i32) -> anyhow::Result<usize> {
    Ok(diesel::update(order::table.find(order_id))
        .set((
            order::in_transit.eq(false),
            order::picked_up.eq(true),
            order::order_number.eq::<Option<i32>>(None),
        ))
        .execute(conn)?)
}

pub fn update_order_new(conn: &SqliteConnection, order_id: i32) -> anyhow::Result<usize> {
    Ok(diesel::update(order::table.find(order_id))
        .set((
            order::in_transit.eq(false),
            order::picked_up.eq(false),
            order::order_number.eq::<Option<i32>>(None),
        ))
        .execute(conn)?)
}

pub fn max_order_number(conn: &SqliteConnection) -> anyhow::Result<i32> {
    Ok(order::table
        .select(diesel::dsl::max(order::order_number))
        .first::<Option<i32>>(conn)?
        .unwrap_or_default())
}

#[cfg(test)]
mod test {

    use diesel::*;
    #[test]
    pub fn get_client_with_name() {
        let conn = super::establish_connection(true);
        let results =
            super::customer_with_name(&conn, "%pie%").expect("Could not find customer with name");
        assert!(results.len() > 0)
    }

    #[test]
    pub fn order_for_customer() {
        let conn = super::establish_connection(true);
        let results = super::orders_for_customer(&conn, 1)
            .expect("Could not find orders for customer with this id");
        assert!(results.len() > 0)
    }

    #[test]
    pub fn pending_orders() {
        let conn = super::establish_connection(true);
        let pending_orders =
            super::all_pending_orders(&conn).expect("Could not retreive pending orders");
        assert!(pending_orders.len() > 0);
    }

    #[test]
    pub fn updating_order() {
        let conn = super::establish_connection(true);

        assert_eq!(super::max_order_number(&conn).unwrap(), 1);
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
        let order = super::update_order_in_transit(&conn, 1, 2);
        assert!(order.is_ok());

        if let Ok(order) = order {
            assert_eq!(order.picked_up, false);
            assert_eq!(order.in_transit, true);
        }

        assert_eq!(super::max_order_number(&conn).unwrap(), 2);

        super::update_order_in_transit(&conn, 1, 1).unwrap();
    }

    #[test]
    pub fn pending() {
        let conn = super::establish_connection(true);
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
