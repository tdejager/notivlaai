import { RouteComponentProps } from '@reach/router';
import React, { useState } from 'react';
import AutoSuggest from 'react-autosuggest';

interface SearchComponentProps {
  getSuggestions: (val: string) => string[];
}

export default function SearchComponent(props: SearchComponentProps & RouteComponentProps) {
  const { getSuggestions } = props;
  const [value, setValue] = useState('');
  const [suggestions, setSuggestion] = useState(['']);

  return (
    <div>
      <AutoSuggest
        renderSuggestion={(s) => <div>{s}</div>}
        suggestions={suggestions}
        onSuggestionsFetchRequested={() => setSuggestion(getSuggestions(value))}
        getSuggestionValue={(s) => s}
        inputProps={{
          placeholder: 'Klantnaam',
          value,
          onChange: (_, { newValue, method }) => {
            setValue(newValue);
          },
        }}
        onSuggestionsClearRequested={() => setSuggestion(suggestions)}
        highlightFirstSuggestion
      />
    </div>
  );
}
