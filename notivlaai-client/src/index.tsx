import * as React from 'react';
import { Router, RouteComponentProps } from '@reach/router';
import ReactDOM from 'react-dom';
import setupStore from './store';
import SearchComponent from './SearchComponent';
import App from './App';

const [useStoreHook] = setupStore();

const mount = document.getElementById('orders');
ReactDOM.render(
  <Router>
    <App path="/" useStore={useStoreHook} />
    <SearchComponent path="/search" />
  </Router>,
  mount
);
