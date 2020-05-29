import { OrderType } from './types';

interface InitializeMessage {
  initialize: [OrderType];
}

interface AddOrderMessage {
  addOrder: OrderType;
}

interface RemoveOrderMessage {
  removeOrder: number;
}

/**
 * All types of messages
 *
 */
export type NotificationMessage = InitializeMessage | AddOrderMessage | RemoveOrderMessage;

/**
 * Type guard for initialize message
 */
export function isInitialize(message: NotificationMessage): message is InitializeMessage {
  if ((message as InitializeMessage).initialize) return true;
  return false;
}

/**
 * Type guard for adding order message
 */
export function isAddOrder(message: NotificationMessage): message is AddOrderMessage {
  if ((message as AddOrderMessage).addOrder) return true;
  return false;
}

/**
 * Type guard for removing order message
 */
export function isRemoveOrder(message: NotificationMessage): message is RemoveOrderMessage {
  if ((message as RemoveOrderMessage).removeOrder) return true;
  return false;
}
