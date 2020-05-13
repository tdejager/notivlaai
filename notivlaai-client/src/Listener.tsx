import * as React from 'react';
import { OrderType, VlaaiType } from './types';

export type AddOrderFn = (order: OrderType) => void;

export default function useTimedListener(
  addOrder: AddOrderFn,
  started: boolean,
  setStarted: (boolean) => void
) {
  React.useEffect(() => {
    if (!started) {
      const testData: OrderType = {
        id: 0,
        clientName: 'Tim de Jager',
        rows: [
          {
            vlaai: VlaaiType.Kers,
            amount: 3,
          },
          {
            vlaai: VlaaiType.Abrikoos,
            amount: 3,
          },
        ],
      };

      const test2 = { ...testData };
      test2.clientName = 'Saskia Winkeler';
      test2.id = 1;
      // Add the first order
      addOrder(testData);
      setStarted(true);

      window.setTimeout(() => {
        addOrder(test2);
      }, 1000);
    }
  }, [started]);
}
