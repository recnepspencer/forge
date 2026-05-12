import { createUploadDiagnostics } from "./line_upload_diagnostics_value.js";
import { createDownloadDiagnostics } from "./line_download_diagnostics_value.js";
import {
  createDeliveredVisibleSelection,
  createInitialVisibleSelection,
  createPatchedVisibleSelection,
  createReloadFulfilledVisibleSelection,
  createRestoredVisibleSelection,
} from "./line_visible_selection.js";

function createInitialLineDiagnostics(
  policy,
  requestDescriptor,
  processing,
  upload,
  download,
  hasVisibleValue,
) {
  return freezeWithVisibleSelection(
    createInitialDiagnosticsShape(
      policy,
      requestDescriptor,
      processing,
      upload,
      download,
      hasVisibleValue,
    ),
    createInitialVisibleSelection(
      requestDescriptor,
      hasVisibleValue,
    ),
  );
}

function freezeWithVisibleSelection(diagnostics, visibleSelection) {
  Object.defineProperty(diagnostics, "visibleSelection", {
    value: visibleSelection,
    enumerable: true,
    configurable: false,
    writable: false,
  });
  return Object.freeze(diagnostics);
}

function createInitialDiagnosticsShape(
  policy,
  requestDescriptor,
  processing,
  upload,
  download,
  hasVisibleValue,
) {
  return {
    policyProfileName: policy.profileName,
    continuity: "preserveVisibleValue",
    freshnessPolicy: policy.staleAfterSettle ? "immediatelyStale" : "stable",
    request: createRequestDiagnostics(requestDescriptor),
    basis: createInitialBasisDiagnostics(requestDescriptor.context.basisId),
    processing,
    upload: createUploadDiagnostics(upload),
    download: createDownloadDiagnostics(download),
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
    deliveryCount: 0,
    lastSupersededOperation: null,
    lastInvalidationCause: null,
    lastInvalidationScope: null,
    lastPatchKind: null,
    lastPatchScope: null,
    lastPatchedItemId: null,
    lastPatchedAspect: null,
    lastPatchedSummary: null,
    lastDeliveryKind: null,
    lastDeliveryScope: null,
    lastDeliveryPacketId: null,
    lastDeliveryBasisId: null,
    lastEffect: null,
    preservedVisibleValueOnLastRejection: false,
    lastTimeoutOperation: null,
    lastErrorMessage: null,
    visibleValueVersion: hasVisibleValue ? 1 : 0,
  };
}

function createReloadFulfilledDiagnostics(
  previous,
  operation,
  processing,
  upload,
  download,
  visibleValueChanged,
  retryAttempts,
) {
  const countsAlreadyAdvanced = previous.pendingOperation === operation;
  return freezeWithVisibleSelection(
    {
      ...previous,
      processing,
      upload: createUploadDiagnostics(upload),
      download: createDownloadDiagnostics(download),
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
    },
    createReloadFulfilledVisibleSelection(
      previous.visibleSelection,
      previous.request,
      operation,
      visibleValueChanged,
    ),
  );
}

function createReloadRejectedDiagnostics(previous, operation, message, retryAttempts) {
  const countsAlreadyAdvanced = previous.pendingOperation === operation;
  return freezeWithVisibleSelection(
    {
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
    },
    previous.visibleSelection,
  );
}

function createPendingReloadDiagnostics(
  previous,
  operation,
  supersededOperation,
) {
  return freezeWithVisibleSelection(
    {
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
    },
    previous.visibleSelection,
  );
}

function createTimedOutReloadDiagnostics(previous, operation, retryAttempts) {
  return freezeWithVisibleSelection(
    {
      ...previous,
      lastOperation: operation,
      lastOutcome: "timedOut",
      pendingOperation: null,
      retryAttemptCount: previous.retryAttemptCount + retryAttempts,
      timeoutCount: previous.timeoutCount + 1,
      preservedVisibleValueOnLastRejection: true,
      lastTimeoutOperation: operation,
      lastErrorMessage: `${operation} timed out`,
    },
    previous.visibleSelection,
  );
}

function createInvalidatedDiagnostics(previous, cause, scope) {
  return freezeWithVisibleSelection(
    {
      ...previous,
      invalidationCount: previous.invalidationCount + 1,
      lastInvalidationCause: cause,
      lastInvalidationScope: scope,
    },
    previous.visibleSelection,
  );
}

function createPatchedDiagnostics(previous, patch, result, effectEnvelope) {
  return freezeWithVisibleSelection(
    {
      ...previous,
      patchCount: previous.patchCount + 1,
      lastPatchKind: patch.kind,
      lastPatchScope: result.scope,
      lastPatchedItemId: result.itemId,
      lastPatchedAspect: result.aspect,
      lastPatchedSummary: result.summary,
      lastEffect: effectEnvelope,
      visibleValueVersion:
        previous.visibleValueVersion + (result.valueChanged ? 1 : 0),
    },
    createPatchedVisibleSelection(
      previous.visibleSelection,
      effectEnvelope,
    ),
  );
}

function createInverseRollbackDiagnostics(previous, rollback, result) {
  return freezeWithVisibleSelection(
    {
      ...previous,
      lastOperation: "restore",
      lastOutcome: "fulfilled",
      pendingOperation: null,
      lastPatchKind: rollback.inverse.patch.kind,
      lastPatchScope: result.scope,
      lastPatchedItemId: result.itemId,
      lastPatchedAspect: result.aspect,
      lastPatchedSummary: result.summary,
      preservedVisibleValueOnLastRejection: false,
      lastTimeoutOperation: null,
      lastErrorMessage: null,
      visibleValueVersion:
        previous.visibleValueVersion + (result.valueChanged ? 1 : 0),
    },
    createRestoredVisibleSelection(
      previous.visibleSelection,
      rollback,
    ),
  );
}

function createDeliveredDiagnostics(previous, delivery) {
  const basisId =
    delivery.nextBasisId === undefined
      ? previous.request.context.basisId
      : delivery.nextBasisId;
  const basis = createDeliveredBasisDiagnostics(
    previous.basis,
    delivery.basisId,
    basisId,
  );
  return freezeWithVisibleSelection(
    {
      ...previous,
      request: Object.freeze({
        ...previous.request,
        context: Object.freeze({
          ...previous.request.context,
          basisId,
        }),
      }),
      basis,
      lastOperation: "delivery",
      lastOutcome: "fulfilled",
      pendingOperation: null,
      patchCount:
        delivery.patchKind === null
          ? previous.patchCount
          : previous.patchCount + 1,
      deliveryCount: previous.deliveryCount + 1,
      supersessionCount:
        previous.supersessionCount
        + (delivery.supersededOperation === null ? 0 : 1),
      lastSupersededOperation: delivery.supersededOperation,
      lastPatchKind: delivery.patchKind,
      lastPatchScope: delivery.patchScope,
      lastPatchedItemId: delivery.patchedItemId,
      lastPatchedAspect: delivery.patchedAspect,
      lastPatchedSummary: delivery.patchedSummary,
      lastDeliveryKind: delivery.deliveryKind,
      lastDeliveryScope: delivery.deliveryScope,
      lastDeliveryPacketId: delivery.packetId,
      lastDeliveryBasisId: delivery.basisId,
      lastEffect: delivery.effectEnvelope,
      preservedVisibleValueOnLastRejection: false,
      lastTimeoutOperation: null,
      lastErrorMessage: null,
      visibleValueVersion:
        previous.visibleValueVersion + (delivery.valueChanged ? 1 : 0),
    },
    createDeliveredVisibleSelection(delivery.effectEnvelope),
  );
}

function createInitialBasisDiagnostics(currentBasisId) {
  return Object.freeze({
    currentBasisId,
    advanceCount: 0,
    lastAdvanceFromBasisId: null,
    lastAdvanceToBasisId: null,
  });
}

function createDeliveredBasisDiagnostics(previous, deliveryBasisId, nextBasisId) {
  if (deliveryBasisId === nextBasisId) {
    return previous;
  }
  return Object.freeze({
    currentBasisId: nextBasisId,
    advanceCount: previous.advanceCount + 1,
    lastAdvanceFromBasisId: deliveryBasisId,
    lastAdvanceToBasisId: nextBasisId,
  });
}

function createRequestDiagnostics(requestDescriptor) {
  return Object.freeze({
    baseUrl: requestDescriptor.baseUrl,
    target: requestDescriptor.target,
    method: requestDescriptor.method,
    bodyPresent: requestDescriptor.body !== null,
    auth: requestDescriptor.auth,
    context: Object.freeze({
      headerNames: Object.freeze(
        Object.keys(requestDescriptor.context.headers).sort(),
      ),
      correlationId: requestDescriptor.context.correlationId,
      branchId: requestDescriptor.context.branchId,
      basisId: requestDescriptor.context.basisId,
    }),
    sources: requestDescriptor.sources,
    continuation: requestDescriptor.continuation,
    processingJob: requestDescriptor.processingJob,
    uploadTransport: requestDescriptor.uploadTransport,
    effects: requestDescriptor.effects,
  });
}

export {
  createInvalidatedDiagnostics,
  createDeliveredDiagnostics,
  createInitialLineDiagnostics,
  createInverseRollbackDiagnostics,
  createPatchedDiagnostics,
  createPendingReloadDiagnostics,
  createReloadFulfilledDiagnostics,
  createReloadRejectedDiagnostics,
  createTimedOutReloadDiagnostics,
};
