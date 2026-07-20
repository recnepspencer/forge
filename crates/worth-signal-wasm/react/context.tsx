import { createContext, createElement, useContext } from "react";
import type { JSX, ReactNode } from "react";

import type { ReactSignalsStore } from "./model.js";

const ReactSignalsStoreContext = createContext<ReactSignalsStore | null>(null);

export function ReactSignalsStoreProvider({
  store,
  children,
}: {
  store: ReactSignalsStore;
  children?: ReactNode;
}): JSX.Element {
  return createElement(
    ReactSignalsStoreContext.Provider,
    { value: store },
    children,
  ) as JSX.Element;
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
