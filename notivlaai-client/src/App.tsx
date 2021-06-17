import * as React from 'react';
import { useEffect } from 'react';
import { UseStore } from 'zustand';
import { animated, useTransition } from 'react-spring';
import { RouteComponentProps } from '@reach/router';
import { OrderComponent, OrderComponentType } from './OrderComponent';
import { OrderContainer } from './components';
import useTimedListener from './Listener';
import { NotivlaaiStore } from './store';
import { isAddOrder, isInitialize, isRemoveOrder } from './messages';
import playBell from "./bell";

interface OrderRoomProps {
  useStore: UseStore<NotivlaaiStore>;
  // Function to set the order as retrieved
  setOrderRetrieved?: (id: number) => void;
  // Are we running a demo?
  demo?: boolean;
  // Is this running in a test?
  isTest?: boolean;
  // Disable all animations
  disableAnimations?: boolean;
}

export default function OrderRoom({
  demo = false,
  isTest = false,
  setOrderRetrieved,
  useStore,
  disableAnimations,
}: OrderRoomProps & RouteComponentProps) {
  const [started, setStarted] = React.useState(false);
  const { orders, removeOrder, replaceOrders, addOrder, notification } = useStore((state) => ({
    notification: state.notification,
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
      if (notification === null) return;
      // Add an order to the store when requested
      if (isAddOrder(notification)) {
        addOrder(notification.addOrder);

        // Play bell sound
        (async() => playBell())()
      }
      // Initialize the list of orders when requested
      else if (isInitialize(notification)) replaceOrders(notification.initialize);
      // Remove an order when requested
      else if (isRemoveOrder(notification)) removeOrder(notification.removeOrder);
      else throw new Error('Cannot decode web-socket message');
    }, [notification]);
  } else {
    // eslint-disable-next-line no-param-reassign
    setOrderRetrieved = removeOrder;
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
            viewType={OrderComponentType.OrderRoom}
            order={element}
            onDelivered={() => setOrderRetrieved(element.id)}
          />
        </animated.div>
      ))
    : orders.map((order) => (
        <OrderComponent
          viewType={OrderComponentType.OrderRoom}
          key={order.id}
          order={order}
          onDelivered={() => setOrderRetrieved(order.id)}
        />
      ));
  return <OrderContainer>{allOrders}</OrderContainer>;
}
