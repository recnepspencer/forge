import { createUploadDiagnostics } from "./line_upload_diagnostics_value.js";

function createInitialLineDiagnostics(
  policy,
  requestDescriptor,
  processing,
  upload,
  hasVisibleValue,
) {
  return Object.freeze({
    policyProfileName: policy.profileName,
    continuity: "preserveVisibleValue",
    freshnessPolicy: policy.staleAfterSettle ? "immediatelyStale" : "stable",
    request: createRequestDiagnostics(requestDescriptor),
    processing,
    upload: createUploadDiagnostics(upload),
    lastOperation: "initialLoad",
    lastOutcome: "fulfilled",
    pendingOperation: null,
    refreshCount: 0,
    revalidateCount: 0,
    retryAttemptCount: 0,
    rejectionCount: 0,
    timeoutCount: 0,
    supersessionCount: 0,
    invalidationCount: 0,
    patchCount: 0,
    lastSupersededOperation: null,
    lastInvalidationCause: null,
    lastInvalidationScope: null,
    lastPatchKind: null,
    lastPatchScope: null,
    lastPatchedItemId: null,
    lastPatchedAspect: null,
    lastPatchedSummary: null,
    preservedVisibleValueOnLastRejection: false,
    lastTimeoutOperation: null,
    lastErrorMessage: null,
    visibleValueVersion: hasVisibleValue ? 1 : 0,
  });
}

function createReloadFulfilledDiagnostics(
  previous,
  operation,
  processing,
  upload,
  visibleValueChanged,
  retryAttempts,
) {
  const countsAlreadyAdvanced = previous.pendingOperation === operation;
  return Object.freeze({
    ...previous,
    processing,
    upload: createUploadDiagnostics(upload),
    lastOperation: operation,
    lastOutcome: "fulfilled",
    pendingOperation: null,
    refreshCount:
      previous.refreshCount
      + (operation === "refresh" && !countsAlreadyAdvanced ? 1 : 0),
    revalidateCount:
      previous.revalidateCount
      + (operation === "revalidate" && !countsAlreadyAdvanced ? 1 : 0),
    retryAttemptCount: previous.retryAttemptCount + retryAttempts,
    preservedVisibleValueOnLastRejection: false,
    lastErrorMessage: null,
    visibleValueVersion:
      previous.visibleValueVersion + (visibleValueChanged ? 1 : 0),
  });
}

function createReloadRejectedDiagnostics(previous, operation, message, retryAttempts) {
  const countsAlreadyAdvanced = previous.pendingOperation === operation;
  return Object.freeze({
    ...previous,
    lastOperation: operation,
    lastOutcome: "rejected",
    pendingOperation: null,
    refreshCount:
      previous.refreshCount
      + (operation === "refresh" && !countsAlreadyAdvanced ? 1 : 0),
    revalidateCount:
      previous.revalidateCount
      + (operation === "revalidate" && !countsAlreadyAdvanced ? 1 : 0),
    retryAttemptCount: previous.retryAttemptCount + retryAttempts,
    rejectionCount: previous.rejectionCount + 1,
    preservedVisibleValueOnLastRejection: true,
    lastTimeoutOperation: null,
    lastErrorMessage: message,
  });
}

function createPendingReloadDiagnostics(
  previous,
  operation,
  supersededOperation,
) {
  return Object.freeze({
    ...previous,
    lastOperation: operation,
    lastOutcome: "pending",
    pendingOperation: operation,
    refreshCount: previous.refreshCount + (operation === "refresh" ? 1 : 0),
    revalidateCount:
      previous.revalidateCount + (operation === "revalidate" ? 1 : 0),
    supersessionCount:
      previous.supersessionCount + (supersededOperation === null ? 0 : 1),
    lastSupersededOperation: supersededOperation,
    preservedVisibleValueOnLastRejection: false,
    lastTimeoutOperation: null,
    lastErrorMessage: null,
  });
}

function createTimedOutReloadDiagnostics(previous, operation, retryAttempts) {
  return Object.freeze({
    ...previous,
    lastOperation: operation,
    lastOutcome: "timedOut",
    pendingOperation: null,
    retryAttemptCount: previous.retryAttemptCount + retryAttempts,
    timeoutCount: previous.timeoutCount + 1,
    preservedVisibleValueOnLastRejection: true,
    lastTimeoutOperation: operation,
    lastErrorMessage: `${operation} timed out`,
  });
}

function createInvalidatedDiagnostics(previous, cause, scope) {
  return Object.freeze({
    ...previous,
    invalidationCount: previous.invalidationCount + 1,
    lastInvalidationCause: cause,
    lastInvalidationScope: scope,
  });
}

function createPatchedDiagnostics(previous, patch, result) {
  return Object.freeze({
    ...previous,
    patchCount: previous.patchCount + 1,
    lastPatchKind: patch.kind,
    lastPatchScope: result.scope,
    lastPatchedItemId: result.itemId,
    lastPatchedAspect: result.aspect,
    lastPatchedSummary: result.summary,
    visibleValueVersion:
      previous.visibleValueVersion + (result.valueChanged ? 1 : 0),
  });
}

function createRequestDiagnostics(requestDescriptor) {
  return Object.freeze({
    auth: requestDescriptor.auth,
    context: Object.freeze({
      headerNames: Object.freeze(
        Object.keys(requestDescriptor.context.headers).sort(),
      ),
      correlationId: requestDescriptor.context.correlationId,
      branchId: requestDescriptor.context.branchId,
      basisId: requestDescriptor.context.basisId,
    }),
    continuation: requestDescriptor.continuation,
    processingJob: requestDescriptor.processingJob,
    uploadTransport: requestDescriptor.uploadTransport,
  });
}

export {
  createInvalidatedDiagnostics,
  createInitialLineDiagnostics,
  createPatchedDiagnostics,
  createPendingReloadDiagnostics,
  createReloadFulfilledDiagnostics,
  createReloadRejectedDiagnostics,
  createTimedOutReloadDiagnostics,
};
