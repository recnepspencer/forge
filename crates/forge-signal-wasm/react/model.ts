export interface ObservationBoundarySummary {
  branchId: number;
  deliveredEventCount: number;
  rollbackSuppressedEventCount: number;
  boundaryEvents: ReadonlyArray<unknown>;
}

export interface WebPerformanceSummary {
  activeHandleCount: number;
  activeCallbackCount: number;
  activeComputeCallbackCount: number;
  activeComputeCollectorCount: number;
  matchedWatcherBreadth: number;
  deliveredObservationCount: number;
  rollbackSuppressedDeliveryCount: number;
  serialExecutorUsageCount: number;
  parallelExecutorUsageCount: number;
  outputSerializationCount: number;
  outputSerializationBreadth: number;
  jsCallbackInvocationCount: number;
  jsCallbackFailureCount: number;
  computeCallbackRegistrationCount: number;
  computeCallbackDisposalCount: number;
  computeCallbackInvocationCount: number;
  computeCallbackFailureCount: number;
  computeCallbackGenerationMismatchDenialCount: number;
  computeCallbackSelfReadDenialCount: number;
  computeCallbackDynamicCycleDenialCount: number;
  computeCallbackPromiseReturnDenialCount: number;
  computeCallbackInvalidReturnDenialCount: number;
  computeCallbackCollectorInstallationCount: number;
  computeCallbackCaptureCount: number;
  computeCallbackCapturedReadCount: number;
  computeCallbackReturnSerializationBreadth: number;
  computeCallbackAllocationCount: number;
  computeCallbackReuseCount: number;
  computeCallbackDependencyPatchCount: number;
  computeCallbackDependencyPatchAddedCount: number;
  computeCallbackDependencyPatchRemovedCount: number;
  computeCallbackDependencyPatchRetainedCount: number;
  computeCallbackRuntimeReadBreadth: number;
  computeCallbackConstantNoSignalReadClassificationCount: number;
  computeCallbackSignalTrackedClassificationCount: number;
  computeCallbackMissingUnavailabilityCount: number;
  compatibilityReadCount: number;
  compatibilityReadBreadth: number;
}

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

export interface WebObservationNotice {
  triggerMatched: boolean;
  meaningfulChange: boolean;
}

export interface DisposableHandleLike {}

export interface SignalHandleLike {
  id: string;
  get(): unknown;
}

export interface SignalsTransactionLike {
  set(input: SignalHandleLike, value: unknown): void;
}

export interface SignalDiagnosticsLike {
  latestObservation(): ObservationBoundarySummary | null;
  latestFlow(): unknown | null;
  performanceSummary(): WebPerformanceSummary;
}

export interface CompatibilityAppLike {
  read(id: string): unknown;
}

export interface SignalsLike {
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

export interface ReactSignalsStore {
  readonly signals: SignalsLike;
  subscribeSignal(signal: SignalHandleLike | string, listener: () => void): () => void;
  getSignalSnapshot(signal: SignalHandleLike | string): unknown;
  subscribeDiagnostics(listener: () => void): () => void;
  getDiagnosticsSnapshot(): SignalsDiagnosticsSnapshot;
  transaction(callback: (tx: SignalsTransactionLike) => void): unknown;
  batch(callback: (tx: SignalsTransactionLike) => void): unknown;
  refreshDiagnostics(): SignalsDiagnosticsSnapshot;
  performanceSummary(): ReactPerformanceSummary;
  dispose(): void;
}
