import * as React from 'react';
import { Router } from '@reach/router';
import ReactDOM from 'react-dom';
import setupStore from './store';
import SearchComponent from './SearchComponent';
import App from './App';
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

const mount = document.getElementById('orders');
ReactDOM.render(
  <Router>
    <App path="/" useStore={useStoreHook} />
    <SearchComponent getSuggestions={getSuggestions} getOrders={getOrders} path="/search" />
  </Router>,
  mount
);
