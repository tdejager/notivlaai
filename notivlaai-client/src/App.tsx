import * as React from 'react';
import { UseStore, State } from 'zustand';
import { OrderType } from './types';
import { OrderComponent } from './OrderComponent';
import { OrderContainer } from './components';

interface AppProps {
  useStore: UseStore<State>;
  disableAnimations?: boolean;
}

export default function App(props: AppProps) {
  const { useStore, disableAnimations } = props;
  const { orders, removeOrder } = useStore((state) => ({
    orders: state.orders,
    removeOrder: state.removeOrder,
  }));

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
