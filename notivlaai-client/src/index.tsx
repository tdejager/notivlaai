import * as React from 'react';
import { Router } from '@reach/router';
import ReactDOM from 'react-dom';
import setupStore from './store';
import SearchComponent from './SearchComponent';
import OrderRoom from './App';
import { OrderType } from './types';

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

// Get orders function
const inTransit = async (id: number) => {
  const response = await fetch(`order/in_transit/${id}`);
  console.log(response);
  if (!response.ok) {
    throw new Error('Cannot set order in transit');
  }
};

const orderRetrieved = async (id: number) => {
  // In case we are faking the API, then do not do the call
  // Ok we have retrieved this order
  const response = await fetch(`order/retrieved/${id}`);
  if (!response.ok) {
    throw new Error('Cannot set order as retrieved');
  }
};

const mount = document.getElementById('orders');
ReactDOM.render(
  <Router>
    <OrderRoom setOrderRetrieved={orderRetrieved} path="/" useStore={useStoreHook} />
    <SearchComponent
      getSuggestions={getSuggestions}
      getOrders={getOrders}
      onInTransit={async (id) => inTransit(id)}
      path="/search"
    />
  </Router>,
  mount
);
