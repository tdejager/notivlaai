import appelImage from './assets/appel.png';
import rijstImage from './assets/rijst.jpeg';
import abrikozenImage from './assets/abrikozenvlaai.png';
import kersenImage from './assets/kersenvlaai.png';
import kruimelPuddingImage from './assets/kruimelpudding.png';
import halfhalfImage from './assets/halfhalf.png';

// Type of vlaai
export enum VlaaiType {
  Abrikoos = 'Abrikoos',
  Kruimelpudding = 'Kruimelpudding',
  HalfHalf = 'HalfHalf',
  Rijst = 'Rijst',
  Kers = 'Kers',
  Appel = 'Appel',
}

// Converts a vlaai enum to the correct image
export function vlaaiToImage(vlaaiType: VlaaiType) {
  switch (vlaaiType) {
    case VlaaiType.Abrikoos:
      return abrikozenImage;
    case VlaaiType.Kers:
      return kersenImage;
    case VlaaiType.Kruimelpudding:
      return kruimelPuddingImage;
    case VlaaiType.HalfHalf:
      return halfhalfImage;
    case VlaaiType.Rijst:
      return rijstImage;
    case VlaaiType.Appel:
      return appelImage;
    default:
      return appelImage;
  }
}

export interface OrderRow {
  vlaai: VlaaiType;
  amount: number;
}

export interface OrderType {
  id: number;
  customerName: string;
  inTransit: boolean;
  pickedUp: boolean;
  rows: Array<OrderRow>;
}
