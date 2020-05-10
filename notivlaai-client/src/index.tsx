import * as React from 'react';
import ReactDOM from 'react-dom';
import { VlaaiType, OrderType } from './types';
import setupStore from './store';
import App from './App';

const testData: OrderType = {
  id: 0,
  clientName: 'Tim de Jager',
  rows: [
    {
      vlaai: VlaaiType.Kers,
      amount: 3,
    },
    {
      vlaai: VlaaiType.Abrikoos,
      amount: 3,
    },
  ],
};

const test2 = { ...testData };
test2.clientName = 'Saskia Winkeler';
test2.id = 1;

const [useStoreHook, api] = setupStore();

// Set some test data
api.setState({ orders: [testData] });
window.setTimeout(() => api.setState({ orders: [testData, test2] }), 1000);

const mount = document.getElementById('orders');

ReactDOM.render(<App useStore={useStoreHook} />, mount);
