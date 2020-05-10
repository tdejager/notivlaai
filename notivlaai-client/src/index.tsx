import * as React from 'react';
import ReactDOM from 'react-dom';
import create, { UseStore, State } from 'zustand';
import { OrderContainer } from './components';
import { OrderProps, OrderComponent } from './OrderComponent';
import { VlaaiType, OrderType } from './types';

const testData: OrderType = {
  id: 0,
  clientName: 'Tim de Jager',
  rows: [
    {
      vlaai: VlaaiType.Kers,
      amount: 3,
    },
    {
      vlaai: VlaaiType.Abrikoos,
      amount: 3,
    },
  ],
};

const test2 = { ...testData };
test2.clientName = 'Saskia Winkeler';
test2.id = 1;

const [useStoreHook, api] = create((set) => ({
  orders: [],
  addOrder: (order: OrderType) => set((state) => ({ orders: [...state.orders, order] })),
  removeOrder: (order: OrderType) =>
    set((state) => ({ orders: [...state.orders.filter((v: OrderType) => v.id !== order.id)] })),
}));

// Set some test data
api.setState({ orders: [testData] });
window.setTimeout(() => api.setState({ orders: [testData, test2] }), 1000);

interface AppProps {
  useStore: UseStore<State>;
}

function App(props: AppProps) {
  const { useStore } = props;
  const { orders, removeOrder } = useStore((state) => ({
    orders: state.orders,
    removeOrder: state.removeOrder,
  }));

  const allOrders = orders.map((value: OrderType) => (
    <OrderComponent key={value.id} order={value} onDelivered={() => removeOrder(value)} />
  ));
  return <OrderContainer>{allOrders}</OrderContainer>;
}
const mount = document.getElementById('orders');

ReactDOM.render(<App useStore={useStoreHook} />, mount);
