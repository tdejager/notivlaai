import { RouteComponentProps } from '@reach/router';
import React, { useState, useEffect } from 'react';
import AutoSuggest from 'react-autosuggest';
import { OrderType } from './types';
import { OrderComponent } from './OrderComponent';
import { OrderContainer } from './components';

interface SearchComponentProps {
  getSuggestions: (val: string) => Promise<[number, string][]>;
  getOrders: (id: number) => Promise<OrderType[]>;
}

export default function SearchComponent(props: SearchComponentProps & RouteComponentProps) {
  const { getSuggestions, getOrders } = props;
  const [value, setValue] = useState('');
  const [orders, setOrders] = useState(Array<OrderType>());
  const [suggestions, setSuggestion] = useState(Array<[number, string]>());

  // Set the data if the value has changed
  useEffect(() => {
    const setData = async () => {
      // Find the customer id
      const f = suggestions.find(([_, name]) => name === value);
      // If it is not undefined
      if (f !== undefined) {
        const [id] = f;
        const newOrders = await getOrders(id);
        console.log('SET', newOrders);
        setOrders(newOrders);
      }
    };

    setData();
  }, [value, suggestions]);

  return (
    <>
      <div>
        <AutoSuggest
          renderSuggestion={(s) => <div>{s[1]}</div>}
          suggestions={suggestions}
          onSuggestionsFetchRequested={async () => setSuggestion(await getSuggestions(value))}
          getSuggestionValue={(s) => s[1]}
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
          <OrderComponent key={order.id} order={order} onDelivered={() => 1} />
        ))}
      </OrderContainer>
    </>
  );
}
