import { create } from 'zustand';
import { OrderType } from './types';

// This is the store that stores the orders
export interface NotivlaaiStore {
  orders: Array<OrderType>;
  // adding orders to the store
  addOrder: (order: OrderType) => void;
  // remove order from the store
  removeOrder: (order: number) => void;
  // replace all orders in the store
  replaceOrders: (orders: [OrderType]) => void;
}

function innerSetupStore({ fakeApi = false }: StoreProps) {
  return create<NotivlaaiStore>((set) => ({
    orders: [],
    addOrder: (order: OrderType) => set((state) => ({ orders: [...state.orders, order] })),
    replaceOrders: (orders: [OrderType]) => set(() => ({ orders: [...orders] })),
    removeOrder: async (id: number) => {
      let isOk = fakeApi;
      // In case we are faking the API, then do not do the call
      if (!fakeApi) {
        // Ok we have retrieved this order
        const response = await fetch(`order/retrieved/${id}`);
        isOk = response.ok;
      }
      if (isOk)
        set((state) => ({ orders: [...state.orders.filter((v: OrderType) => v.id !== id)] }));
    },
  }));
}

/**
 * Props that can be passed into the store
 */
interface StoreProps {
  fakeApi?: boolean;
}

/**
 * Setup a test store for testing functionality
 */
export function setupTestStore() {
  return innerSetupStore({ fakeApi: true });
}

/**
 * Setup a default store for actual use
 */
export default function setupStore() {
  return innerSetupStore({});
}
