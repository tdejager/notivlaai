import { create } from 'zustand';
import { OrderType } from './types';

// This is the store that stores the orders
export interface NotivlaaiStore {
  orders: Array<OrderType>;
  addOrder: (order: OrderType) => void;
  removeOrder: (order: number) => void;
  replaceOrders: (orders: [OrderType]) => void;
}

export default function setupStore() {
  return create<NotivlaaiStore>((set) => ({
    orders: [],
    addOrder: (order: OrderType) => set((state) => ({ orders: [...state.orders, order] })),
    replaceOrders: (orders: [OrderType]) => set(() => ({ orders: [...orders] })),
    removeOrder: async (id: number) => {
      set((state) => ({ orders: [...state.orders.filter((v: OrderType) => v.id !== id)] }));
    },
  }));
}
