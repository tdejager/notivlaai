import * as React from 'react';
import ReactDOM from 'react-dom';
import create from 'zustand';
import { OrderContainer, VlaaiType } from './components';
import { OrderProps, OrderComponent } from './OrderComponent';

const testData: OrderProps = {
  clientName: 'Tim de Jager',
  rows: [
    {
      vlaai: VlaaiType.Kers,
      amount: 3
    },
    {
      vlaai: VlaaiType.Abrikoos,
      amount: 3
    }
  ]
};

const [useStore, api] = create(set => ({
  allOrders: [],
  addOrder: (order: OrderProps) => set(state => ({ orders: [...state.orders, order] }))
}));

// Set some test data
api.setState({ allOrders: [testData] });

function AllOrders() {
  const order: [OrderProps] = useStore(state => state.allOrders);
  const allOrders = order.map(value => (
    <OrderComponent key={value.clientName} clientName={value.clientName} rows={value.rows} />
  ));
  return <OrderContainer>{allOrders}</OrderContainer>;
}
const mount = document.getElementById('orders');

ReactDOM.render(<AllOrders> </AllOrders>, mount);
