import * as React from 'react';
import { useEffect } from 'react';
import { UseStore, State } from 'zustand';
import { OrderType, VlaaiType } from './types';
import { OrderComponent } from './OrderComponent';
import { OrderContainer } from './components';
import useTimedListener from './Listener';
import createWebSocketWrapper from './createWebSocketWrapper';
import { NotivlaaiStore } from './store';

interface AppProps {
  useStore: UseStore<NotivlaaiStore>;
  demo?: boolean;
  disableAnimations?: boolean;
}

interface InitializeMessage {
  initialize: [OrderType];
}

interface AddOrder {
  addOrder: OrderType;
}

interface RemoveOrder {
  removeOrder: OrderType;
}

type AllMessages = InitializeMessage | AddOrder | RemoveOrder;

export default function App({ demo = false, useStore, disableAnimations }: AppProps) {
  const [started, setStarted] = React.useState(false);
  const { orders, removeOrder, replaceOrders, addOrder } = useStore((state) => ({
    orders: state.orders,
    removeOrder: state.removeOrder,
    replaceOrders: state.replaceOrders,
    addOrder: state.addOrder,
  }));

  // Use the demo effect
  if (demo) {
    useTimedListener(addOrder, started, setStarted);
  } else {
    // Use an actual web socket
    useEffect(() => {
      if (!started) {
        const webSocketWrapper = createWebSocketWrapper('ws://127.0.0.1:9001');
        webSocketWrapper.onMessage((e) => {
          const data = JSON.parse(e.data);
          console.log(data);
          if ('initialize' in Object.keys(data)) {
            const message = data as InitializeMessage;
            replaceOrders(message.initialize);
          }
        });
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
