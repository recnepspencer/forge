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
