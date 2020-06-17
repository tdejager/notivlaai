import 'normalize.css';
import styled from 'styled-components';

export const OrderContainer = styled.div`
  display: grid;
  grid-auto-rows: auto;
  grid-gap: 5vmin 0px;
  color: white;
`;

export const Order = styled.div`
  display: grid;
  justify-items: center;
  background-color: #3c3836;
  grid-gap: 2vmin 0px;
  border-radius: 10px;
`;

export const Vlaaien = styled.div`
  display: grid;
  margin: 0 auto;
  grid-gap 0 4vmin;
  justify-items: center;
  grid-template-columns: 1fr 1fr 1fr;
  max-width: 95%;
`;

export const BestellingHeader = styled.h3`
  color: #ebdbb2;
  text-align: center;
`;

export const Button = styled.button`
  background-color: #458588; /* Blue */
  border: 1px solid #a89984;
  border-radius: 3px;
  display: inline-flex;
  flex: 0 0 auto;
  flex-direction: row;
  justify-content: center;
  align-self: center;
  color: #ebdbb2;
  padding: 15px 32px;
  text-align: center;
  font-size: 16px;
  &:hover {
    transition: color 0.2s, background-color 0.2s;
    color: white;
    background-color: #b8bb26;
  }
  &:active {
    transition: color 0.1s, background-color 0.1s;
    color: #ebdbb2;
    background-color: #282828;
  }
  &:disabled {
    background-color: gray;
    color: black;
  }
`;
Button.displayName = 'Button';

export const Vlaai = styled.img`
  max-width: 100%;
  max-height: 30vmin;
  object-fit: contain;
  border: 1px solid #fbf1c7;
  border-radius: 10px;
`;
