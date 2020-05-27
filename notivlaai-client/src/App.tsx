import * as React from 'react';
import { useEffect } from 'react';
import { UseStore } from 'zustand';
import { useTransition, animated } from 'react-spring';
import { RouteComponentProps } from '@reach/router';
import { OrderType } from './types';
import { OrderComponent } from './OrderComponent';
import { OrderContainer } from './components';
import useTimedListener from './Listener';
import createWebSocketWrapper from './createWebSocketWrapper';
import { NotivlaaiStore } from './store';

interface OrderRoomProps {
  useStore: UseStore<NotivlaaiStore>;
  demo?: boolean;
  // Is this running in a test?
  isTest?: boolean;
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

export default function OrderRoom({
  demo = false,
  isTest = false,
  useStore,
  disableAnimations,
}: OrderRoomProps & RouteComponentProps) {
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
  } else if (!isTest) {
    // Use an actual web socket, we are not in demo and not in test
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

  const transition = useTransition(orders, {
    // Start at bottom of the page
    from: { opacity: 0, transform: 'translate3d(0, 150%, 0)' },
    // Move to the normal position
    enter: { opacity: 1, transform: 'translate3d(0, 0%, 0)' },
    // When leaving fade-out
    leave: { opacity: 0 },
  });

  const allOrders = !disableAnimations
    ? transition((style, element) => (
        <animated.div style={style}>
          <OrderComponent
            key={element.id}
            order={element}
            onDelivered={() => removeOrder(element.id)}
          />
        </animated.div>
      ))
    : orders.map((order) => (
        <OrderComponent key={order.id} order={order} onDelivered={() => removeOrder(order.id)} />
      ));
  return <OrderContainer>{allOrders}</OrderContainer>;
}
