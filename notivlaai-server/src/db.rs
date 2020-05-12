use crate::schema::*;
use diesel::*;
use diesel::{Connection, SqliteConnection};

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
}

#[derive(Insertable)]
#[table_name = "vlaai_to_order"]
pub struct NewVlaaiToOrder {
    pub order_id: i32,
    pub vlaai_id: i32,
    pub amount: i32,
}

pub fn establish_connection() -> SqliteConnection {
    dotenv::dotenv().ok();

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn customer_with_name<T: AsRef<str>>(
    conn: &SqliteConnection,
    name: T,
) -> Result<Vec<Customer>, Box<dyn std::error::Error>> {
    Ok(customer::table
        .filter(customer::first_name.like(name.as_ref()))
        .load(conn)?)
}

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
}
