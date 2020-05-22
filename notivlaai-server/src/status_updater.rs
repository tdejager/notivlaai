use crate::db::PendingOrder;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::sync::mpsc;

/// The message that can be received by
/// someone subscribing on the updater
pub enum UpdateOrder {
    /// Remove an order from the screen
    OrderRetrieved(u32),
    /// Add an order to the screen
    AddOrder(PendingOrder),
}

pub struct OrderStatusUpdater {
    /// Publishes order updates
    publisher: Sender<UpdateOrder>,
    /// Receives order updates to process
    receiver: mpsc::Receiver<UpdateOrder>,
}

/// Keep running to collect orders
pub struct OrderRunner {
    /// Receives order updates to process
    receiver: mpsc::Receiver<UpdateOrder>,
    /// Publishes order updates
    publisher: Sender<UpdateOrder>,
}

impl OrderRunner {
    /// Receive updates and publishes these over the broadcaster
    pub async fn run(mut self) {
        loop {
            while let Some(value) = self.receiver.recv().await {
                // Do nothing in case of ok or an error, just keep on sending
                if self.publisher.send(value).is_ok() {}
            }
        }
    }
}

pub struct OrderSubscriber {
    /// Publisher, used to give out new subscriptions
    publisher: Sender<UpdateOrder>,
}

impl OrderSubscriber {
    /// Subscribe to a new order subscription
    /// This make use of the tokio channels
    pub fn subscribe(&self) -> Receiver<UpdateOrder> {
        self.publisher.subscribe()
    }
}

impl OrderStatusUpdater {
    pub fn new(receiver: mpsc::Receiver<UpdateOrder>) -> OrderStatusUpdater {
        // This is the async channel
        let (sender, _) = channel(100);

        OrderStatusUpdater {
            publisher: sender,
            receiver,
        }
    }

    /// Subscribe to get order mutator
    /// can send messages to mutate orders in the database
    /// and provides a struct that gives out subscriptions
    pub fn order_mutator(self) -> (OrderSubscriber, OrderRunner) {
        // Create a subscriber part
        let sub = OrderSubscriber {
            publisher: self.publisher.clone(),
        };

        // Create a runner part
        let runner = OrderRunner {
            receiver: self.receiver,
            publisher: self.publisher,
        };
        (sub, runner)
    }
}
