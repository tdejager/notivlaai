use crate::db;
use serde::Serialize;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::sync::mpsc;

/// The message that can be received by
/// someone subscribing on the updater
pub enum UpdateOrder {
    /// Remove an order from the screen
    OrderRetrieved(u32),
    /// Order is in transit
    OrderInTransit(u32),
}

/// This enum signifies published changes to the order
#[derive(Serialize, Clone)]
pub enum OrderPublish {
    /// Add an order to the screen
    AddOrder(db::PendingOrder),
    /// Remove an existing order from the screen
    RemoveOrder(u32),
}

/// Defines an OrderRunner backend that can be abstracted over, so we can have
/// a database backend and a vector backend
pub trait Backend {
    /// Tell the backend to update the order
    fn order_in_transit(
        &mut self,
        id: u32,
    ) -> Result<db::Order, Box<dyn std::error::Error + Send + Sync>>;

    /// Tell the backend that the order has been retrieved
    fn order_retrieved(&mut self, id: u32) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// Convert an order to a pending order
    fn to_pending(
        &self,
        order: db::Order,
    ) -> Result<db::PendingOrder, Box<dyn std::error::Error + Send + Sync>>;
}

/// This updates with regards to the datase
pub struct DBBackend {
    conn: db::PooledConnection,
}

impl Default for DBBackend {
    fn default() -> Self {
        DBBackend {
            conn: db::establish_connection(),
        }
    }
}

impl Backend for DBBackend {
    fn order_in_transit(
        &mut self,
        id: u32,
    ) -> Result<db::Order, Box<dyn std::error::Error + Send + Sync>> {
        db::update_order_in_transit(&self.conn, id as i32)
    }
    fn order_retrieved(&mut self, id: u32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        db::update_order_retrieved(&self.conn, id as i32)?;
        Ok(())
    }
    fn to_pending(
        &self,
        order: db::Order,
    ) -> Result<db::PendingOrder, Box<dyn std::error::Error + Send + Sync>> {
        db::to_pending(&self.conn, order)
    }
}

pub struct OrderStatusUpdater<T> {
    /// Publishes order updates
    publisher: Sender<OrderPublish>,
    /// Receives order updates to process
    receiver: mpsc::Receiver<UpdateOrder>,
    /// Backend to process order updates
    backend: T,
}

/// Keep running to collect orders
pub struct OrderRunner<T> {
    /// Receives order updates to process
    receiver: mpsc::Receiver<UpdateOrder>,
    /// Publishes order updates
    publisher: Sender<OrderPublish>,
    /// Backend to process order updates
    backend: T,
}

impl<T: Backend> OrderRunner<T> {
    /// Receive updates and publishes these over the broadcaster
    pub async fn run(mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        loop {
            while let Some(value) = self.receiver.recv().await {
                let value = match value {
                    UpdateOrder::OrderRetrieved(id) => {
                        self.backend.order_retrieved(id)?;
                        // Remove this order from the screen
                        OrderPublish::RemoveOrder(id)
                    }
                    UpdateOrder::OrderInTransit(id) => {
                        let order = self.backend.order_in_transit(id)?;
                        // Add a new order to the screen
                        OrderPublish::AddOrder(self.backend.to_pending(order)?)
                    }
                };
                // Do nothing in case of ok or an error, just keep on sending
                if self.publisher.send(value).is_ok() {}
            }
        }
    }
}

/// Gives out subscriptions to receive updates to orders
pub struct OrderSubscriber {
    /// Publisher, used to give out new subscriptions
    publisher: Sender<OrderPublish>,
}

impl OrderSubscriber {
    /// Subscribe to a new order subscription
    /// This make use of the tokio channels
    pub fn subscribe(&self) -> Receiver<OrderPublish> {
        self.publisher.subscribe()
    }
}

impl<T: Backend + Default> OrderStatusUpdater<T> {
    pub fn new(receiver: mpsc::Receiver<UpdateOrder>) -> OrderStatusUpdater<T> {
        // This is the async channel
        let (sender, _) = channel(100);

        OrderStatusUpdater {
            publisher: sender,
            receiver,
            backend: Default::default(),
        }
    }

    /// Subscribe to get order mutator
    /// can send messages to mutate orders in the database
    /// and provides a struct that gives out subscriptions
    pub fn order_mutator(self) -> (OrderSubscriber, OrderRunner<T>) {
        // Create a subscriber part
        let sub = OrderSubscriber {
            publisher: self.publisher.clone(),
        };

        // Create a runner part
        let runner = OrderRunner {
            receiver: self.receiver,
            publisher: self.publisher,
            backend: self.backend,
        };
        (sub, runner)
    }
}

#[cfg(test)]
mod tests {

    use super::UpdateOrder;
    use crate::db;
    use std::collections::HashMap;

    struct TestBackend {
        orders: HashMap<u32, db::Order>,
    }

    impl Default for TestBackend {
        fn default() -> Self {
            let mut map = HashMap::new();
            map.insert(
                1,
                db::Order {
                    id: 1,
                    customer_id: 1,
                    in_transit: false,
                    picked_up: false,
                },
            );
            Self { orders: map }
        }
    }

    // Backend for simple testing
    impl super::Backend for TestBackend {
        fn order_in_transit(
            &mut self,
            id: u32,
        ) -> Result<db::Order, Box<dyn std::error::Error + Send + Sync>> {
            let order = self.orders.get_mut(&id).ok_or("Not there")?;
            order.picked_up = false;
            order.in_transit = true;
            Ok(*order)
        }
        fn order_retrieved(
            &mut self,
            id: u32,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let order = self.orders.get_mut(&id).ok_or("Not there")?;
            order.picked_up = true;
            order.in_transit = false;
            Ok(())
        }
        fn to_pending(
            &self,
            order: db::Order,
        ) -> Result<db::PendingOrder, Box<dyn std::error::Error + Send + Sync>> {
            Ok(db::PendingOrder {
                id: order.id as u32,
                customer_name: "Piet".to_string(),
                rows: Default::default(),
            })
        }
    }

    #[tokio::test]
    async fn test_update() {
        let (mut sender, receiver) = tokio::sync::mpsc::channel(100);
        let order_updater = super::OrderStatusUpdater::<TestBackend>::new(receiver);
        let (subscriber, runner) = order_updater.order_mutator();
        let mut receiver = subscriber.subscribe();

        // Run the runner
        tokio::spawn(async { runner.run().await });

        // Set that the order is in transit
        assert!(sender.send(UpdateOrder::OrderInTransit(1)).await.is_ok());

        // Expect to get an update
        let publish_update = receiver.recv().await.unwrap();

        if let super::OrderPublish::AddOrder(o) = publish_update {
            assert_eq!(o.id, 1);
        } else {
            panic!("Did not get the correct response")
        }

        // Set that the order has been picked up
        assert!(sender.send(UpdateOrder::OrderRetrieved(1)).await.is_ok());

        // Expect to get an update
        let publish_update = receiver.recv().await.unwrap();

        if let super::OrderPublish::RemoveOrder(id) = publish_update {
            assert_eq!(id, 1)
        } else {
            panic!("Did not get the correct response")
        }
    }
}
