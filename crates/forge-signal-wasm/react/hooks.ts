import { useMemo, useSyncExternalStore } from "react";

import type { ReactSignalsStore, SignalHandleLike, SignalsDiagnosticsSnapshot } from "./model.js";

export function useSignalValue<T = unknown>(
  signal: SignalHandleLike,
  store: ReactSignalsStore,
): T {
  const subscribe = useMemo(
    () => (listener: () => void) => store.subscribeSignal(signal, listener),
    [signal, store],
  );
  const getSnapshot = useMemo(
    () => () => store.getSignalSnapshot(signal) as T,
    [signal, store],
  );
  return useSyncExternalStore(subscribe, getSnapshot, getSnapshot);
}

export function useOutputValue<T = unknown>(
  output: SignalHandleLike,
  store: ReactSignalsStore,
): T {
  const subscribe = useMemo(
    () => (listener: () => void) => store.subscribeSignal(output, listener),
    [output, store],
  );
  const getSnapshot = useMemo(
    () => () => store.getSignalSnapshot(output) as T,
    [output, store],
  );
  return useSyncExternalStore(subscribe, getSnapshot, getSnapshot);
}

export function useSignalsDiagnostics(
  store: ReactSignalsStore,
): SignalsDiagnosticsSnapshot {
  return useSyncExternalStore(
    store.subscribeDiagnostics,
    store.getDiagnosticsSnapshot,
    store.getDiagnosticsSnapshot,
  );
}
