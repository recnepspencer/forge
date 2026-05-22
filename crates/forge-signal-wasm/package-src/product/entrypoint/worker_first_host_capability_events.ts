export function recordIgnoredStaleEvent(
  performanceSummary,
  diagnosticsRecorder,
  descriptor,
  invalidationMode,
  currentState,
) {
  performanceSummary.hostCapabilityStaleInvalidationIgnoredCount += 1;
  diagnosticsRecorder.push({
    kind: "InvalidationIgnoredStale",
    family: descriptor.family,
    registrationId: descriptor.registrationId,
    compatibility: descriptor.compatibility,
    invalidationMode,
    queuedInvalidationCount: 1,
    previousState: currentState,
    nextState: currentState,
    touchedNodes: 0,
    reevaluatedNodes: 0,
  });
}

export function recordNoOpEvent(
  performanceSummary,
  diagnosticsRecorder,
  descriptor,
  invalidationMode,
  previousState,
  nextState,
) {
  performanceSummary.hostCapabilityNoOpInvalidationSuppressedCount += 1;
  diagnosticsRecorder.push({
    kind: "InvalidationNoOpSuppressed",
    family: descriptor.family,
    registrationId: descriptor.registrationId,
    compatibility: descriptor.compatibility,
    invalidationMode,
    queuedInvalidationCount: 1,
    previousState,
    nextState,
    touchedNodes: 0,
    reevaluatedNodes: 0,
  });
}

export function recordFlushedEvent(
  performanceSummary,
  diagnosticsRecorder,
  descriptor,
  invalidationMode,
  previousState,
  nextState,
  touchedNodes = 0,
  reevaluatedNodes = 0,
) {
  performanceSummary.hostCapabilityInvalidationBatchFlushCount += 1;
  diagnosticsRecorder.push({
    kind: "InvalidationFlushed",
    family: descriptor.family,
    registrationId: descriptor.registrationId,
    compatibility: descriptor.compatibility,
    invalidationMode,
    queuedInvalidationCount: 1,
    previousState,
    nextState,
    touchedNodes,
    reevaluatedNodes,
  });
}

export function recordHostDependencyRefreshFailed(
  performanceSummary,
  diagnosticsRecorder,
  descriptor,
  invalidationMode,
  error,
) {
  performanceSummary.hostCapabilityUnavailabilityArtifactCount += 1;
  performanceSummary.hostCapabilityDependencyRefreshFailureCount =
    (performanceSummary.hostCapabilityDependencyRefreshFailureCount ?? 0) + 1;
  diagnosticsRecorder.push({
    kind: "HostDependencyRefreshFailed",
    family: descriptor.family,
    registrationId: descriptor.registrationId,
    compatibility: descriptor.compatibility,
    invalidationMode,
    queuedInvalidationCount: 1,
    previousState: null,
    nextState: null,
    touchedNodes: 0,
    reevaluatedNodes: 0,
    failureMessage: error instanceof Error ? error.message : String(error),
  });
}

export function recordHostCapabilityReadDenied(
  performanceSummary,
  diagnosticsRecorder,
  descriptor,
  error,
  denialReason,
  deniedBeforePublication,
) {
  performanceSummary.hostCapabilityUnavailabilityArtifactCount += 1;
  performanceSummary.hostCapabilityReadDenialCount =
    (performanceSummary.hostCapabilityReadDenialCount ?? 0) + 1;
  diagnosticsRecorder.push({
    kind: "HostCapabilityReadDenied",
    family: descriptor.family,
    registrationId: descriptor.registrationId,
    compatibility: descriptor.compatibility,
    invalidationMode: "callback-capture",
    queuedInvalidationCount: 0,
    previousState: null,
    nextState: null,
    touchedNodes: 0,
    reevaluatedNodes: 0,
    errorCode: error?.code ?? "computeCallbackHostCapabilityReadDenied",
    denialReason,
    deniedBeforePublication,
    failureMessage: error instanceof Error ? error.message : String(error),
  });
}
