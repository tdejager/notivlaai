import * as React from 'react';
import { Router } from '@reach/router';
import ReactDOM from 'react-dom';
import setupStore from './store';
import SearchComponent from './SearchComponent';
import App from './App';

const [useStoreHook] = setupStore();

const mount = document.getElementById('orders');
ReactDOM.render(
  <Router>
    <App path="/" useStore={useStoreHook} />
    <SearchComponent getSuggestions={() => ['hallo', 'doei']} path="/search" />
  </Router>,
  mount
);
