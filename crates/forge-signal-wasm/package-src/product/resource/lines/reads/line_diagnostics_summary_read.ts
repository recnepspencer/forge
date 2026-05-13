import { readLineHistoryAvailability } from "./line_history_availability_read.js";
import {
  readMutationResponseSummaryDigest,
} from "../../mutation/resource_mutation_response_diagnostics_projection.js";

function readLineDiagnosticsSummary(materialization) {
  const diagnostics = materialization.binding.diagnosticsSignal();
  const status = materialization.binding.statusSignal();
  const freshness = materialization.binding.freshnessSignal();
  const visibleValue = materialization.binding.valueSignal();
  const history = readLineHistoryAvailability(materialization);
  const mutationResponseSummaryDigest =
    readMutationResponseSummaryDigest(diagnostics);
  const current = {
    status,
    freshness,
    hasVisibleValue: visibleValue !== null,
    visibleValueVersion: diagnostics.visibleValueVersion,
  };
  Object.defineProperty(current, "visibleSelection", {
    value: diagnostics.visibleSelection,
    enumerable: false,
    configurable: false,
    writable: false,
  });
  return Object.freeze({
    current: Object.freeze(current),
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
      deliveryCount: diagnostics.deliveryCount,
      basisAdvanceCount: diagnostics.basis.advanceCount,
    }),
    latest: Object.freeze({
      invalidationCause: diagnostics.lastInvalidationCause,
      invalidationScope: diagnostics.lastInvalidationScope,
      patchKind: diagnostics.lastPatchKind,
      patchScope: diagnostics.lastPatchScope,
      patchedItemId: diagnostics.lastPatchedItemId,
      patchedField: diagnostics.lastPatchedField,
      patchedRegion: diagnostics.lastPatchedRegion,
      patchedPath: diagnostics.lastPatchedPath,
      patchedAspect: diagnostics.lastPatchedAspect,
      patchedSummary: diagnostics.lastPatchedSummary,
      deliveryKind: diagnostics.lastDeliveryKind,
      deliveryScope: diagnostics.lastDeliveryScope,
      deliveryPacketId: diagnostics.lastDeliveryPacketId,
      deliveryBasisId: diagnostics.lastDeliveryBasisId,
      basisCurrentId: diagnostics.basis.currentBasisId,
      basisAdvanceFromId: diagnostics.basis.lastAdvanceFromBasisId,
      basisAdvanceToId: diagnostics.basis.lastAdvanceToBasisId,
      effect: diagnostics.lastEffect,
      supersededOperation: diagnostics.lastSupersededOperation,
      timeoutOperation: diagnostics.lastTimeoutOperation,
      errorMessage: diagnostics.lastErrorMessage,
      preservedVisibleValueOnLastRejection:
        diagnostics.preservedVisibleValueOnLastRejection,
      ...(mutationResponseSummaryDigest === null
        ? {}
        : {
            mutationResponsePlanId: mutationResponseSummaryDigest.planId,
            mutationResponseTargetCount: mutationResponseSummaryDigest.targetCount,
            mutationResponseConfirmationKind:
              mutationResponseSummaryDigest.confirmationKind,
            mutationResponseConfirmationDigest:
              mutationResponseSummaryDigest.confirmationDigest,
            mutationResponseRollbackDigest:
              mutationResponseSummaryDigest.rollbackDigest,
            mutationResponseMergeRebaseDigest:
              mutationResponseSummaryDigest.mergeRebaseDigest,
            mutationResponseExecutionDigest:
              mutationResponseSummaryDigest.executionDigest,
            mutationResponseDiagnosticCount:
              mutationResponseSummaryDigest.diagnosticCount,
            mutationResponseDiagnosticDigest:
              mutationResponseSummaryDigest.diagnosticDigest,
            mutationResponsePlanCount: mutationResponseSummaryDigest.planCount,
          }),
    }),
    request: diagnostics.request,
    processing: diagnostics.processing,
    upload: diagnostics.upload,
    download: diagnostics.download,
    explainability: history.availability,
  });
}

export { readLineDiagnosticsSummary };
