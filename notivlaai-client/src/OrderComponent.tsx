import * as React from 'react';
import styled from 'styled-components';
import { VlaaiType, vlaaiToImage, OrderType } from './types';
import { Button, Order, BestellingHeader, Vlaai, Vlaaien } from './components';

const OverflowDiv = styled.div`
  overflow-wrap: anywhere;
`;
const SubText = styled.p`
  margin: 0;
  padding: 0;
  text-align: center;
`;
/**
 * This is the display that show a single vlaai in the order
 */
function VlaaiDisplay(props: { vlaai: VlaaiType; amount: number }) {
  const { vlaai, amount } = props;

  return (
    <OverflowDiv>
      <Vlaai src={vlaaiToImage(vlaai)} />
      <SubText>
        {' '}
        {amount}x {vlaai.toString()}{' '}
      </SubText>
    </OverflowDiv>
  );
}

export interface OrderProps {
  order: OrderType;
  onDelivered: () => void;
}

/**
 * This is a single order for a client
 */
export function OrderComponent(props: OrderProps) {
  const { order, onDelivered } = props;
  const { customerName } = order;

  const displayOrders = order.rows.map((value) => {
    return <VlaaiDisplay key={value.vlaai.toString()} vlaai={value.vlaai} amount={value.amount} />;
  });

  return (
    <Order>
      <BestellingHeader>Bestelling voor {customerName}:</BestellingHeader>
      <Vlaaien>{displayOrders}</Vlaaien>
      <Button onClick={() => onDelivered()}>
        <span role="img" aria-label="check">
          ✔️&nbsp;
        </span>
        <span>Opgehaald!</span>
      </Button>
    </Order>
  );
}
