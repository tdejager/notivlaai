import { create } from 'zustand';
import { OrderType } from './types';

export interface NotivlaaiStore {
  orders: Array<OrderType>;
  addOrder: (order: OrderType) => void;
  removeOrder: (order: OrderType) => void;
  replaceOrders: (orders: [OrderType]) => void;
}

export default function setupStore() {
  return create<NotivlaaiStore>((set) => ({
    orders: [],
    addOrder: (order: OrderType) =>
      set((state) => ({ ...state, orders: [...state.orders, order] })),
    replaceOrders: (orders: [OrderType]) => set(() => ({ orders: [...orders] })),
    removeOrder: (order: OrderType) =>
      set((state) => ({ orders: [...state.orders.filter((v: OrderType) => v.id !== order.id)] })),
  }));
}
