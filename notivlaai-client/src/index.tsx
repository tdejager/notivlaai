import * as React from 'react';
import ReactDOM from 'react-dom';
import { Order, OrderContainer, VlaaiType } from './components';
import { OrderProps, OrderComponent } from './OrderComponent';

const testData: OrderProps = {
  clientName: 'Tim de Jager',
  orders: [
    {
      vlaai: VlaaiType.Kers,
      amount: 3
    }
  ]
};

const orders = (
  <OrderContainer>
    <OrderComponent clientName={testData.clientName} orders={testData.orders} />
    <Order>This is a second order </Order>
  </OrderContainer>
);

const mount = document.getElementById('orders');

ReactDOM.render(orders, mount);
