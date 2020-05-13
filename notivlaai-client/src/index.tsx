import * as React from 'react';
import ReactDOM from 'react-dom';
import { VlaaiType, OrderType } from './types';
import setupStore from './store';
import App from './App';

const [useStoreHook, api] = setupStore();

const mount = document.getElementById('orders');
ReactDOM.render(<App useStore={useStoreHook} />, mount);
