use crate::db::PendingOrder;
use std::sync::mpsc::{channel as sync_channel, Receiver as SyncReceiver, Sender as SyncSender};
use tokio::sync::broadcast::{channel, Receiver, Sender};

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
    /// synchronous receiver for updating order
    sync_recv: SyncReceiver<UpdateOrder>,
    /// Sender to be able to clone to receive values
    sync_sender: SyncSender<UpdateOrder>,
}

pub struct OrderStatusSub {
    order_updates: Receiver<UpdateOrder>,
    order_changes: Sender<UpdateOrder>,
}

impl OrderStatusUpdater {
    pub fn new() -> OrderStatusUpdater {
        // This is the async channel
        let (sender, _) = channel(100);

        let (sync_sender, sync_recv) = sync_channel();

        OrderStatusUpdater {
            publisher: sender,
            sync_recv,
            sync_sender,
        }
    }

    /// Subscribe to a new order subscription
    /// This make use of the tokio channels
    pub fn subscribe(&self) -> Receiver<UpdateOrder> {
        self.publisher.subscribe()
    }

    pub fn get_updater(&self) -> SyncSender<UpdateOrder> {
        self.sync_sender.clone()
    }

    pub async fn run(&self) {
        loop {
            let result = self.sync_recv.try_recv();
            match result {
                Ok(update_order) => {
                    // Do something here
                }
                Err(_) => {}
            }
        }
    }
}
