import React, { createContext, useContext } from "react";

import type { ReactSignalsStore } from "./model.js";

const ReactSignalsStoreContext = createContext<ReactSignalsStore | null>(null);

export function ReactSignalsStoreProvider({
  store,
  children,
}: {
  store: ReactSignalsStore;
  children?: React.ReactNode;
}): React.JSX.Element {
  return React.createElement(
    ReactSignalsStoreContext.Provider,
    { value: store },
    children,
  ) as React.JSX.Element;
}

export function useReactSignalsStore(): ReactSignalsStore {
  const store = useMaybeReactSignalsStore();
  if (store === null) {
    throw new TypeError(
      "React signals store was not provided. Wrap the tree with <ReactSignalsStoreProvider store={...}> or pass store explicitly.",
    );
  }
  return store;
}

export function useMaybeReactSignalsStore(): ReactSignalsStore | null {
  return useContext(ReactSignalsStoreContext);
}
