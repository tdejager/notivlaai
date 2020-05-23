import * as React from 'react';
import { useEffect } from 'react';
import { UseStore } from 'zustand';
import { OrderType } from './types';
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

type InitializeMessage = {
  initialize: [OrderType];
};

interface AddOrderMessage {
  addOrder: OrderType;
}

interface RemoveOrderMessage {
  removeOrder: number;
}

type AllMessage = InitializeMessage | AddOrderMessage | RemoveOrderMessage;

/**
 * Type guard for initialize message
 */
function isInitialize(message: AllMessage): message is InitializeMessage {
  if ((message as InitializeMessage).initialize) return true;
  return false;
}

/**
 * Type guard for adding order message
 */
function isAddOrder(message: AllMessage): message is AddOrderMessage {
  if ((message as AddOrderMessage).addOrder) return true;
  return false;
}

/**
 * Type guard for removing order message
 */
function isRemoveOrder(message: AllMessage): message is RemoveOrderMessage {
  if ((message as RemoveOrderMessage).removeOrder) return true;
  return false;
}

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
          const messageJson = JSON.parse(e.data);
          // Add an order to the store when requested
          if (isAddOrder(messageJson)) addOrder(messageJson.addOrder);
          // Initialize the list of orders when requested
          else if (isInitialize(messageJson)) replaceOrders(messageJson.initialize);
          // Remove an order when requested
          else if (isRemoveOrder(messageJson)) removeOrder(messageJson.removeOrder);
          else throw new Error('Cannot decode web-socket message');
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
      onDelivered={() => removeOrder(value.id)}
      disableAnimations={disableAnimations}
    />
  ));
  return <OrderContainer>{allOrders}</OrderContainer>;
}
