import 'normalize.css';
import styled from 'styled-components';
import vlaaiImage from '../assets/vlaai.jpg';

export const OrderContainer = styled.div`
  display: grid;
  grid-auto-rows: auto;
  grid-gap: 5vmin 0px;
  color: white;
`;

export const Order = styled.div`
  display: grid;
  grid-template-rows: 1fr 3fr 2fr;
  background-color: #3c3836;
  border-radius: 10px;
`;

export const Vlaaien = styled.div`
  display: grid;
  grid-auto-flow: column;
  margin: 0 auto;
  grid-gap 0 4vmin;
  justify-items: center;
  max-width: 95%;
`;

export const BestellingHeader = styled.h3`
  color: #ebdbb2;
  text-align: center;
`;

// export const Vlaai = styled.div`
// object-fit: cover;
// min-width: 100%;
// min-height: 100%;
// background-image: url(${vlaai});
// background-size: cover;
// `;
// Vlaai.displayName = 'Vlaai';

export const Vlaai = styled.img`
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  border: 1px solid #fbf1c7;
  border-radius: 10px;
`;

export enum VlaaiType {
  Abrikoos,
  Kruimelpudding,
  HalfHalf,
  Kers
}

// Converts a vlaai enum to the correct image
export function vlaaiToImage(vlaaiType: VlaaiType) {
  switch (vlaaiType) {
    case VlaaiType.Abrikoos:
    case VlaaiType.HalfHalf:
    case VlaaiType.Kers:
    case VlaaiType.Kruimelpudding:
      return vlaaiImage;
    default:
      return vlaaiImage;
  }
}
