// import uuidv1 from 'uuid/v1';

// import createWebSocketWrapper, { Unsubscribe } from '../web-socket-wrapper/createWebSocketWrapper';

// type UnixTimeSeconds = Brand<number, 'UnixTimeSeconds'>;
// type NanoSeconds = Brand<number, 'NanoSeconds'>;
// type RelativityTime = [UnixTimeSeconds, NanoSeconds];

// enum MessageType {
//   SubscribeToDataWindow = 'SubscribeToDataWindow',
//   UnsubscribeFromDataWindow = 'UnsubscribeFromDataWindow',
//   VlaaiOrdered = 'VlaaiOrdered',
//   DataPointAdded = 'DataPointAdded',
// }

// /**
//  * Name of a channel
//  */
// export type ChannelName = Brand<string, 'ChannelName'>;

// /**
//  * A data point
//  */
// export interface DataPoint {
//   subscriptionId: SubscriptionId;
//   timestamp: Date;
//   data: any;
// }

// interface Message<T extends MessageType, U> {
//   type: T;
//   payload: U;
// }

// interface Handler<T> {
//   (payload: T): void;
// }

// type VlaaiOrderedMessage = Message<MessageType.VlaaiOrdered, ChannelName>;

// /**
//  * Handles when a channel is added
//  */
// export type VlaaiOrderedHandler = Handler<ChannelName>;

// /**
//  *
//  */
// export type Window = {
//   start: Date;
//   end: Date;
// };

// /**
//  *
//  */
// export default function createRelativityClient() {
//   const webSocket = createWebSocketWrapper('ws://localhost:9001');

//   return {
//     /**
//      *
//      */
//     get isConnected(): boolean {
//       return webSocket.isConnected;
//     },

//     /**
//      *
//      */
//     async connect(): Promise<void> {
//       await webSocket.connect();
//     },

//     /**
//      *
//      */
//     subscribeToDataWindow(window: Window): SubscriptionId {
//       const subscriptionId = generateSubscriptionId();

//       webSocket.send({
//         type: MessageType.SubscribeToDataWindow,
//         payload: {
//           subscriptionId,
//           start: toRelativityTime(window.start),
//           end: toRelativityTime(window.end),
//         },
//       });

//       return subscriptionId;
//     },

//     /**
//      *
//      *
//      * @param subscriptionId
//      */
//     unsubscribe(subscriptionId: SubscriptionId): void {
//       webSocket.send({
//         type: MessageType.UnsubscribeFromDataWindow,
//         payload: subscriptionId,
//       });
//     },

//     /**
//      *
//      *
//      * @param handler
//      */
//     onVlaaiOrdered(handler: VlaaiOrderedHandler): Unsubscribe {
//       return webSocket.onMessage((event) => {
//         const data = JSON.parse(event.data);
//         if (data.type === MessageType.VlaaiOrdered) {
//           handler((data as VlaaiOrderedMessage).payload);
//         }
//       });
//     },

//     /**
//      * Register a message handler for every data point
//      *
//      * @param handler
//      */
//     onDataPointAdded(handler: DataPointAddedHandler): Unsubscribe {
//       return webSocket.onMessage((event) => {
//         const eventData = JSON.parse(event.data);
//         if (eventData.type === MessageType.DataPointAdded) {
//           const { payload } = eventData as DataPointAddedMessage;
//           const { subscriptionId, timestamp, data } = payload;
//           handler({
//             subscriptionId,
//             timestamp: fromRelativityTime(timestamp),
//             data,
//           });
//         }
//       });
//     },
//   };
// }

// export type RelativityClient = ReturnType<typeof createRelativityClient>;
