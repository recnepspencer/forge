import { readLineHistoryAvailability } from "./line_history_availability_read.js";

function readLineDiagnosticsSummary(materialization) {
  const diagnostics = materialization.binding.diagnosticsSignal();
  const status = materialization.binding.statusSignal();
  const freshness = materialization.binding.freshnessSignal();
  const visibleValue = materialization.binding.valueSignal();
  const history = readLineHistoryAvailability(materialization);
  return Object.freeze({
    current: Object.freeze({
      status,
      freshness,
      hasVisibleValue: visibleValue !== null,
      visibleValueVersion: diagnostics.visibleValueVersion,
    }),
    activity: Object.freeze({
      lastOperation: diagnostics.lastOperation,
      lastOutcome: diagnostics.lastOutcome,
      pendingOperation: diagnostics.pendingOperation,
      continuity: diagnostics.continuity,
      freshnessPolicy: diagnostics.freshnessPolicy,
    }),
    counts: Object.freeze({
      refreshCount: diagnostics.refreshCount,
      revalidateCount: diagnostics.revalidateCount,
      retryAttemptCount: diagnostics.retryAttemptCount,
      rejectionCount: diagnostics.rejectionCount,
      timeoutCount: diagnostics.timeoutCount,
      supersessionCount: diagnostics.supersessionCount,
      invalidationCount: diagnostics.invalidationCount,
      patchCount: diagnostics.patchCount,
    }),
    latest: Object.freeze({
      invalidationCause: diagnostics.lastInvalidationCause,
      invalidationScope: diagnostics.lastInvalidationScope,
      patchKind: diagnostics.lastPatchKind,
      patchScope: diagnostics.lastPatchScope,
      patchedItemId: diagnostics.lastPatchedItemId,
      patchedAspect: diagnostics.lastPatchedAspect,
      patchedSummary: diagnostics.lastPatchedSummary,
      supersededOperation: diagnostics.lastSupersededOperation,
      timeoutOperation: diagnostics.lastTimeoutOperation,
      errorMessage: diagnostics.lastErrorMessage,
      preservedVisibleValueOnLastRejection:
        diagnostics.preservedVisibleValueOnLastRejection,
    }),
    request: diagnostics.request,
    processing: diagnostics.processing,
    upload: diagnostics.upload,
    explainability: history.availability,
  });
}

export { readLineDiagnosticsSummary };
