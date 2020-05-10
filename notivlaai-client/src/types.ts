import vlaaiImage from './assets/vlaai.jpg';

// Type of vlaai
export enum VlaaiType {
  Abrikoos = 'Abrikoos',
  Kruimelpudding = 'Kruimelpudding',
  HalfHalf = 'HalfHalf',
  Kers = 'Kers',
  Appel = 'Appel',
}

// Converts a vlaai enum to the correct image
export function vlaaiToImage(vlaaiType: VlaaiType) {
  switch (vlaaiType) {
    case VlaaiType.Abrikoos:
    case VlaaiType.HalfHalf:
    case VlaaiType.Kers:
    case VlaaiType.Appel:
    case VlaaiType.Kruimelpudding:
      return vlaaiImage;
    default:
      return vlaaiImage;
  }
}

export interface OrderRow {
  vlaai: VlaaiType;
  amount: number;
}

export interface OrderType {
  id: number;
  clientName: string;
  rows: Array<OrderRow>;
}
