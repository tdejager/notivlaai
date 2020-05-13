import * as React from 'react';
import { UseStore, State } from 'zustand';
import { OrderType } from './types';
import { OrderComponent } from './OrderComponent';
import { OrderContainer } from './components';
import useTimedListener from './Listener';

interface AppProps {
  useStore: UseStore<State>;
  demo?: boolean;
  disableAnimations?: boolean;
}

export default function App({ demo = true, useStore, disableAnimations }: AppProps) {
  const [started] = React.useState(true);
  const { orders, removeOrder, addOrder } = useStore((state) => ({
    orders: state.orders,
    removeOrder: state.removeOrder,
    addOrder: state.addOrder,
  }));

  if (demo) useTimedListener(addOrder, started);

  const allOrders = orders.map((value: OrderType) => (
    <OrderComponent
      key={value.id}
      order={value}
      onDelivered={() => removeOrder(value)}
      disableAnimations={disableAnimations}
    />
  ));
  return <OrderContainer>{allOrders}</OrderContainer>;
}
