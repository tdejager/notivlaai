export interface MessageHandler {
  (e: MessageEvent): void;
}

export interface Unsubscribe {
  (): void;
}

/**
 *
 *
 * @param url
 */
export default function createWebSocketWrapper(url: string) {
  let webSocket: WebSocket | null = null;
  const messageHandlers: MessageHandler[] = [];

  // Connect the message handlers to the web socket
  const connectMessageHandlers = () => {
    if (webSocket === null) throw new Error('can not find WebSocket instance');

    webSocket.onmessage = (e) => {
      messageHandlers.forEach((handler) => handler(e));
    };
  };

  const wrapper = {
    /**
     *
     */
    get isConnected() {
      if (webSocket === null) return false;
      return webSocket.readyState === WebSocket.OPEN;
    },

    /**
     *
     */
    async connect(): Promise<Event> {
      const event = await new Promise<Event>((resolve, reject) => {
        webSocket = new WebSocket(url);
        webSocket.onopen = resolve;
        webSocket.onclose = reject;
        webSocket.onerror = reject;
      });

      connectMessageHandlers();

      return event;
    },

    /**
     *
     */
    send(data: object) {
      if (webSocket) {
        webSocket.send(JSON.stringify(data));
      }
    },

    /**
     * Add a message handler.
     *
     * Returns a function with which to unsubscribe
     *
     * @param handler
     */
    onMessage(handler: MessageHandler): Unsubscribe {
      messageHandlers.push(handler);
      const handlerIndex = messageHandlers.length - 1;
      return () => delete messageHandlers[handlerIndex];
    },
  };

  return wrapper;
}
