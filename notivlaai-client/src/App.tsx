import * as React from 'react';
import { UseStore, State } from 'zustand';
import { OrderType } from './types';
import { OrderComponent } from './OrderComponent';
import { OrderContainer } from './components';
import useTimedListener from './Listener';
import { useEffect } from 'react';
import createWebSocketWrapper from './createWebSocketWrapper';

interface AppProps {
  useStore: UseStore<State>;
  demo?: boolean;
  disableAnimations?: boolean;
}

export default function App({ demo = false, useStore, disableAnimations }: AppProps) {
  const [started, setStarted] = React.useState(false);
  const { orders, removeOrder, addOrder } = useStore((state) => ({
    orders: state.orders,
    removeOrder: state.removeOrder,
    addOrder: state.addOrder,
  }));

  if (demo) {
    useTimedListener(addOrder, started, setStarted);
  } else {
    useEffect(() => {
      if (!started) {
        const webSocketWrapper = createWebSocketWrapper('ws://127.0.0.1:9001');
        webSocketWrapper.onMessage((e) => console.log(e));
        webSocketWrapper
          .connect()
          .then(() => setStarted(true))
          .catch((errr) => console.error(errr));
      }
    }, [started]);
  }

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
