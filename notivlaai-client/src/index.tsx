import * as React from 'react';
import ReactDOM from 'react-dom';
import setupStore from './store';
import App from './App';

const [useStoreHook] = setupStore();

const mount = document.getElementById('orders');
ReactDOM.render(<App useStore={useStoreHook} />, mount);
