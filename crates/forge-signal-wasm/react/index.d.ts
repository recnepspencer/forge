import type {
  ComputedSignal,
  InputSignal,
  ObservationBoundarySummary,
  OutputSignal,
  RunSummary,
  Signals,
  SignalsTransaction,
  WebPerformanceSummary,
} from "../forge_signal_wasm.js";

export interface SignalsDiagnosticsSnapshot {
  latestObservation: ObservationBoundarySummary | null;
  latestFlow: unknown | null;
  performanceSummary: WebPerformanceSummary;
}

export interface ReactPerformanceSummary {
  activeSignalSubscriptionCount: number;
  activeReactSubscriberCount: number;
  activeRuntimeWatchHandleCount: number;
  diagnosticsSubscriberCount: number;
  sharedFanoutRatio: number;
}

export interface ReactSignalsStore {
  readonly signals: Signals;
  subscribeSignal(
    signal: InputSignal | ComputedSignal | OutputSignal | string,
    listener: () => void,
  ): () => void;
  getSignalSnapshot(signal: InputSignal | ComputedSignal | OutputSignal | string): unknown;
  subscribeDiagnostics(listener: () => void): () => void;
  getDiagnosticsSnapshot(): SignalsDiagnosticsSnapshot;
  transaction(callback: (tx: SignalsTransaction) => void): RunSummary;
  batch(callback: (tx: SignalsTransaction) => void): RunSummary;
  refreshDiagnostics(): SignalsDiagnosticsSnapshot;
  performanceSummary(): ReactPerformanceSummary;
  dispose(): void;
}

export declare function createReactSignalsStore(signals: Signals): ReactSignalsStore;

export declare function useSignalValue<T = unknown>(
  signal: InputSignal | ComputedSignal,
  store: ReactSignalsStore,
): T;

export declare function useOutputValue<T = unknown>(
  output: OutputSignal,
  store: ReactSignalsStore,
): T;

export declare function useSignalsDiagnostics(
  store: ReactSignalsStore,
): SignalsDiagnosticsSnapshot;
