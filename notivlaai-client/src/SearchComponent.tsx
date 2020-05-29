import { RouteComponentProps } from '@reach/router';
import React, { useState, useEffect } from 'react';
import AutoSuggest from 'react-autosuggest';
import { UseStore } from 'zustand';
import { OrderType } from './types';
import { OrderComponent, OrderComponentType } from './OrderComponent';
import { OrderContainer } from './components';
import { NotivlaaiStore } from './store';

interface SearchComponentProps {
  getSuggestions: (val: string) => Promise<[number, string][]>;
  getOrders: (id: number) => Promise<OrderType[]>;
  onInTransit: (id: number) => void;
  useStore: UseStore<NotivlaaiStore>;
}

export default function SearchComponent(props: SearchComponentProps & RouteComponentProps) {
  const { getSuggestions, getOrders, onInTransit, useStore } = props;
  const [value, setValue] = useState('');
  const [orders, setOrders] = useState(Array<OrderType>());
  const [suggestions, setSuggestion] = useState(Array<[number, string]>());
  const { notification } = useStore((state) => ({ notification: state.notification }));

  // Set the data if the value has changed
  useEffect(() => {
    const setData = async () => {
      // Find the customer id
      const f = suggestions.find(([_, name]) => name === value);
      // If it is not undefined
      if (f !== undefined) {
        const [id] = f;
        const newOrders = await getOrders(id);
        setOrders(newOrders);
      }
    };

    setData();
  }, [value, suggestions, notification]);

  return (
    <>
      <div>
        <AutoSuggest
          renderSuggestion={([id, name]) => <div>{name}</div>}
          suggestions={suggestions}
          onSuggestionsFetchRequested={async () => setSuggestion(await getSuggestions(value))}
          getSuggestionValue={([id, name]) => name}
          inputProps={{
            placeholder: 'Klantnaam',
            value,
            onChange: (_, { newValue }) => {
              setValue(newValue);
            },
          }}
          onSuggestionsClearRequested={() => setSuggestion(suggestions)}
          highlightFirstSuggestion
        />
      </div>
      <OrderContainer style={{ marginTop: '3vmin' }}>
        {orders.map((order) => (
          <OrderComponent
            viewType={OrderComponentType.Search}
            key={order.id}
            order={order}
            onInTransit={() => {
              onInTransit(order.id);
            }}
            onDelivered={() => 1}
          />
        ))}
      </OrderContainer>
    </>
  );
}
