import * as React from 'react';
// import ReactDOM from 'react-dom';
import { VlaaiType, Order, BestellingHeader, Vlaai, Vlaaien, vlaaiToImage } from './components';

/**
 * This is the display that show a single vlaai in the order
 */
function VlaaiDisplay(props: { vlaai: VlaaiType; amount: number }) {
  const { vlaai, amount } = props;
  const style: React.CSSProperties = {
    overflowWrap: 'anywhere'
  };

  const subTextStyle: React.CSSProperties = {
    margin: '0',
    padding: '0',
    textAlign: 'center'
  };

  return (
    <div style={style}>
      <Vlaai src={vlaaiToImage(vlaai)} />
      <p style={subTextStyle}>
        {' '}
        {amount}x {vlaai.toString()}{' '}
      </p>
    </div>
  );
}

export interface OrderProps {
  clientName: string;
  orders: [{ vlaai: VlaaiType; amount: number }];
}
/**
 * This is a single order for a client
 */
export function OrderComponent(props: OrderProps) {
  const { clientName, orders } = props;

  const displayOrders = orders.map(value => {
    return VlaaiDisplay(value);
  });

  return (
    <Order>
      <BestellingHeader>Bestelling voor {clientName}:</BestellingHeader>
      <Vlaaien>{displayOrders}</Vlaaien>
    </Order>
  );
}
