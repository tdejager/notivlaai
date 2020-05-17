import * as React from 'react';
// import ReactDOM from 'react-dom';
import styled from 'styled-components';
import { useSpring, animated } from 'react-spring';
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
  disableAnimations?: boolean;
}

const AnimatedOrder = animated(Order);
/**
 * This is a single order for a client
 */
export function OrderComponent(props: OrderProps) {
  const { order, onDelivered, disableAnimations } = props;
  const { customerName } = order;

  const [leave, setLeave] = React.useState(false);

  // This is used for animation
  const style = !leave
    ? useSpring({
        from: { opacity: 0, transform: 'translate3d(0, 150%, 0)' },
        transform: 'translate3d(0, 0%, 0)',
        opacity: 1,
      })
    : useSpring({
        from: { opacity: 1 },
        opacity: 0,
        onRest: onDelivered,
      });

  const displayOrders = order.rows.map((value) => {
    return <VlaaiDisplay key={value.vlaai.toString()} vlaai={value.vlaai} amount={value.amount} />;
  });

  return (
    <AnimatedOrder style={style}>
      <BestellingHeader>Bestelling voor {customerName}:</BestellingHeader>
      <Vlaaien>{displayOrders}</Vlaaien>
      <Button onClick={() => (!disableAnimations ? setLeave(true) : onDelivered())}>
        <span role="img" aria-label="check">
          ✔️&nbsp;
        </span>
        <span>Opgehaald!</span>
      </Button>
    </AnimatedOrder>
  );
}
