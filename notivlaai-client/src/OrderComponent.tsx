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

export enum OrderComponentType {
  Search,
  OrderRoom,
}

export interface OrderProps {
  order: OrderType;
  viewType: OrderComponentType;
  onDelivered?: () => void;
  onInTransit?: () => void;
}

function statusToText(inTransit: boolean, pickedUp: boolean) {
  if (inTransit) return 'Wordt nu afgehaald';
  if (pickedUp) return 'Reeds opgehaald';
  if (!inTransit && !pickedUp) return 'Kan worden opgehaald';
  return 'Onbekende status';
}

function statusToImg(inTransit: boolean, pickedUp: boolean) {
  if (inTransit) return '🚚';
  if (pickedUp) return '✔️';
  if (!inTransit && !pickedUp) return '📦';
  return 'Onbekende status';
}

function buttonsFor(
  viewType: OrderComponentType,
  inTransit: boolean,
  pickedUp: boolean,
  onDelivered?: () => void,
  onInTransit?: () => void
) {
  switch (viewType) {
    // Show this when we are in the order room
    case OrderComponentType.OrderRoom:
      return (
        <Button onClick={() => onDelivered()}>
          <span role="img" aria-label="check">
            ✔️&nbsp;
          </span>
          <span>Opgehaald!</span>
        </Button>
      );
    // Show this when we are searching for something
    case OrderComponentType.Search:
      return (
        <>
          <Button disabled={pickedUp || inTransit} onClick={() => onInTransit()}>
            <span role="img" aria-label="check">
              {statusToImg(inTransit, pickedUp)}&nbsp;
            </span>
            <span>{statusToText(inTransit, pickedUp)}</span>
          </Button>
        </>
      );
    default:
      return <> </>;
  }
}

/**
 * This is a single order for a client
 */
export function OrderComponent(props: OrderProps) {
  const { order, onDelivered, onInTransit, viewType } = props;
  const { customerName, pickedUp, inTransit } = order;

  const displayOrders = order.rows.map((value) => {
    return <VlaaiDisplay key={value.vlaai.toString()} vlaai={value.vlaai} amount={value.amount} />;
  });
  return (
    <Order>
      <BestellingHeader>Bestelling voor {customerName}:</BestellingHeader>
      <Vlaaien>{displayOrders}</Vlaaien>
      {buttonsFor(viewType, inTransit, pickedUp, onDelivered, onInTransit)}
    </Order>
  );
}
