import * as React from 'react';
import { useEffect, useState } from 'react';
import { Router } from '@reach/router';
import ReactDOM from 'react-dom';
import setupStore from './store';
import SearchComponent from './SearchComponent';
import OrderRoom from './App';
import { OrderType } from './types';
import createWebSocketWrapper from './createWebSocketWrapper';
import { NotificationMessage } from './messages';

// Setup the zustand store
const [useStoreHook] = setupStore();

// Get suggestions function
const getSuggestions = async (find: string) => {
  const response = await fetch(`customer/find/${find}`);
  if (response.ok) {
    const json = await response.json();
    return json as [number, string][];
  }

  return [];
};

// Get orders function
const getOrders = async (id: number) => {
  const response = await fetch(`order/find/${id}`);
  if (response.ok) {
    const json = await response.json();
    return json as OrderType[];
  }

  return [];
};

// Set in transit function
const inTransit = async (id: number) => {
  const response = await fetch(`order/in_transit/${id}`);
  if (!response.ok) {
    throw new Error('Cannot set order in transit');
  }
};

// Set order as retrieved
const orderRetrieved = async (id: number) => {
  // Ok we have retrieved this order
  const response = await fetch(`order/retrieved/${id}`);
  if (!response.ok) {
    throw new Error('Cannot set order as retrieved');
  }
};

function Application() {
  const [started, setStarted] = useState(false);
  const { notify } = useStoreHook((state) => ({ notify: state.notify }));

  // Set the websocket
  useEffect(() => {
    if (!started) {
      const { location } = window;
      const { hostname } = location;
      const url = `ws://${hostname}:9001`;
      console.log(url);
      const webSocketWrapper = createWebSocketWrapper(url);
      webSocketWrapper.onMessage((e) => {
        const messageJson = JSON.parse(e.data);
        // Add the message as a notification
        notify(messageJson as NotificationMessage);
      });
      webSocketWrapper
        .connect()
        .then(() => setStarted(true))
        .catch((errr) => console.error(errr));
    }
  });

  return (
    <Router>
      <OrderRoom setOrderRetrieved={orderRetrieved} path="/" useStore={useStoreHook} />
      <SearchComponent
        useStore={useStoreHook}
        getSuggestions={getSuggestions}
        getOrders={getOrders}
        onInTransit={async (id) => inTransit(id)}
        path="/search"
      />
    </Router>
  );
}

const mount = document.getElementById('orders');
ReactDOM.render(<Application />, mount);
