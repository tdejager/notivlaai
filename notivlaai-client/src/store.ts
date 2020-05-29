import { create } from 'zustand';
import { OrderType } from './types';
import { NotificationMessage } from "./messages";

// This is the store that stores the orders
export interface NotivlaaiStore {
  // All the orders in the OrderRoom
  orders: Array<OrderType>;
  // The last notification received
  notification: NotificationMessage;
  // notify
  notify: (notificationMessage: NotificationMessage) => void;
  // adding orders to the store
  addOrder: (order: OrderType) => void;
  // remove order from the store
  removeOrder: (order: number) => void;
  // replace all orders in the store
  replaceOrders: (orders: [OrderType]) => void;
}

function innerSetupStore() {
  return create<NotivlaaiStore>((set) => ({
    orders: [],
    notification: null,
    notify: (notificationMessage: NotificationMessage) => set((state) => ({notification: notificationMessage})),
    addOrder: (order: OrderType) => set((state) => ({ orders: [...state.orders, order] })),
    replaceOrders: (orders: [OrderType]) => set(() => ({ orders: [...orders] })),
    removeOrder: async (id: number) => {
      set((state) => ({ orders: [...state.orders.filter((v: OrderType) => v.id !== id)] }));
    },
  }));
}


/**
 * Setup a default store for actual use
 */
export default function setupStore() {
  return innerSetupStore();
}
