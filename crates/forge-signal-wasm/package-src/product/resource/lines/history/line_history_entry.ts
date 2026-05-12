function createLineHistoryEntry(
  event,
  status,
  freshness,
  diagnostics,
  overrides = {},
) {
  const entry = {
    sequence: 0,
    event,
    status,
    freshness,
    lastOperation: diagnostics.lastOperation,
    lastOutcome: diagnostics.lastOutcome,
    pendingOperation: diagnostics.pendingOperation,
    statusContinuity:
      "continuity" in status ? status.continuity : null,
    retryAttemptCount: diagnostics.retryAttemptCount,
    rejectionCount: diagnostics.rejectionCount,
    timeoutCount: diagnostics.timeoutCount,
    supersessionCount: diagnostics.supersessionCount,
    invalidationCount: diagnostics.invalidationCount,
    patchCount: diagnostics.patchCount,
    deliveryCount: diagnostics.deliveryCount,
    lastSupersededOperation: diagnostics.lastSupersededOperation,
    lastInvalidationCause: diagnostics.lastInvalidationCause,
    lastInvalidationScope: diagnostics.lastInvalidationScope,
    lastPatchKind: diagnostics.lastPatchKind,
    lastPatchScope: diagnostics.lastPatchScope,
    lastPatchedItemId: diagnostics.lastPatchedItemId,
    lastPatchedAspect: diagnostics.lastPatchedAspect,
    lastPatchedSummary: diagnostics.lastPatchedSummary,
    lastDeliveryKind: diagnostics.lastDeliveryKind,
    lastDeliveryScope: diagnostics.lastDeliveryScope,
    lastDeliveryPacketId: diagnostics.lastDeliveryPacketId,
    lastDeliveryBasisId: diagnostics.lastDeliveryBasisId,
    lastEffect: diagnostics.lastEffect,
    currentBasisId: diagnostics.basis.currentBasisId,
    basisAdvanceCount: diagnostics.basis.advanceCount,
    lastBasisAdvanceFromId: diagnostics.basis.lastAdvanceFromBasisId,
    lastBasisAdvanceToId: diagnostics.basis.lastAdvanceToBasisId,
    downloadCount: diagnostics.download.count,
    readyDownloadCount: diagnostics.download.readyCount,
    unavailableDownloadCount: diagnostics.download.unavailableCount,
    incompatibleDownloadCount: diagnostics.download.incompatibleCount,
    preservedVisibleValueOnLastRejection:
      diagnostics.preservedVisibleValueOnLastRejection,
    lastTimeoutOperation: diagnostics.lastTimeoutOperation,
    lastErrorMessage: diagnostics.lastErrorMessage,
    visibleValueVersion: diagnostics.visibleValueVersion,
    supersededOperation: null,
    ...overrides,
  };
  Object.defineProperty(entry, "visibleSelection", {
    value: diagnostics.visibleSelection,
    enumerable: true,
    configurable: false,
    writable: false,
  });
  return Object.freeze(entry);
}

export { createLineHistoryEntry };
