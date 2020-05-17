import create from 'zustand';
import { OrderType } from './types';

export default function setupStore() {
  return create((set) => ({
    orders: [],
    addOrder: (order: OrderType) => set((state) => ({ orders: [...state.orders, order] })),
    replaceOrders: (orders: [OrderType]) => set(() => ({ orders: [...orders] })),
    removeOrder: (order: OrderType) =>
      set((state) => ({ orders: [...state.orders.filter((v: OrderType) => v.id !== order.id)] })),
  }));
}
