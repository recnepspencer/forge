import type {
  FlowSurfaceSummary,
  ObservationSurfaceSummary,
  WebPerformanceSummary,
} from "../types/diagnostics.js";

export interface SignalsDiagnosticsSnapshot {
  latestObservation: ObservationSurfaceSummary | null;
  latestFlow: FlowSurfaceSummary | null;
  performanceSummary: WebPerformanceSummary;
}

export interface ReactPerformanceSummary {
  activeSignalSubscriptionCount: number;
  activeReactSubscriberCount: number;
  activeRuntimeWatchHandleCount: number;
  diagnosticsSubscriberCount: number;
  sharedFanoutRatio: number;
}

export interface WebObservationNotice {
  triggerMatched: boolean;
  meaningfulChange: boolean;
}

export interface DisposableHandleLike {
  free(): void;
}

export interface SignalHandleLike {
  id: string;
  get(): unknown;
}

export interface SignalsTransactionLike {
  set(input: SignalHandleLike, value: unknown): void;
}

export interface SignalDiagnosticsLike {
  latestObservation(): ObservationSurfaceSummary | null;
  latestFlow(): FlowSurfaceSummary | null;
  performanceSummary(): WebPerformanceSummary;
  subscribe(listener: () => void): DisposableHandleLike;
}

export interface CompatibilityAppLike {
  read(id: string): unknown;
}

export interface SignalsLike {
  read(target: SignalHandleLike | string): unknown;
  watch(
    target: SignalHandleLike | string,
    callback: (notice: WebObservationNotice) => void,
  ): DisposableHandleLike;
  nuke(handle: DisposableHandleLike): boolean;
  diagnostics(): SignalDiagnosticsLike;
  compatibilityApp(): CompatibilityAppLike;
  transaction(callback: (tx: SignalsTransactionLike) => void): unknown;
  batch(callback: (tx: SignalsTransactionLike) => void): unknown;
}

export interface ReactSignalsStore<TSignals extends SignalsLike = SignalsLike> {
  readonly signals: TSignals;
  subscribeSignal(signal: SignalHandleLike | string, listener: () => void): () => void;
  getSignalSnapshot(signal: SignalHandleLike | string): unknown;
  subscribeDiagnostics(listener: () => void): () => void;
  getDiagnosticsSnapshot(): SignalsDiagnosticsSnapshot;
  transaction(callback: Parameters<TSignals["transaction"]>[0]): unknown;
  batch(callback: Parameters<TSignals["batch"]>[0]): unknown;
  refreshDiagnostics(): SignalsDiagnosticsSnapshot;
  performanceSummary(): ReactPerformanceSummary;
  dispose(): void;
}
