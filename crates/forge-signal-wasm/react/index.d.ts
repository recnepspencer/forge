import type {
  ReactSignalsStore,
  SignalHandleLike,
  SignalsDiagnosticsSnapshot,
  SignalsLike,
} from "./model.js";

export type { ReactPerformanceSummary, ReactSignalsStore, SignalsDiagnosticsSnapshot } from "./model.js";

export declare function createReactSignalsStore<TSignals extends SignalsLike>(
  signals: TSignals,
): ReactSignalsStore<TSignals>;

export declare function useSignalValue<T = unknown>(
  signal: SignalHandleLike,
  store: ReactSignalsStore,
): T;

export declare function useOutputValue<T = unknown>(
  output: SignalHandleLike,
  store: ReactSignalsStore,
): T;

export declare function useSignalsDiagnostics(
  store: ReactSignalsStore,
): SignalsDiagnosticsSnapshot;
