import { stableValueDigest } from "../values/value_paths.js";

export function readFormResourceSettlementReport(status, freshness, visibleSelection, mutationResponse) {
  if (status.kind === "pending") {
    return pendingSettlement(status, freshness, visibleSelection);
  }
  if (status.kind === "rejected" || status.kind === "timedOut") {
    return failedSettlement(status, freshness, visibleSelection);
  }
  if (mutationResponse !== null) {
    return confirmedSettlement(status, freshness, visibleSelection, mutationResponse);
  }
  return noneSettlement(status, freshness, visibleSelection);
}

function pendingSettlement(status, freshness, visibleSelection) {
  return freezeSettlement({
    kind: "pending",
    operation: status.operation,
    failureKind: null,
    continuity: status.continuity,
    confirmationKind: null,
    freshnessKind: freshness.kind,
    freshnessReason: freshness.kind === "stale" ? freshness.reason : null,
    visibleSelectionKind: visibleSelection.kind,
    branchProof: visibleSelection.branchProof,
    rebaseProof: visibleSelection.rebaseProof,
    message: null,
    retryRecommended: false,
    retryOperation: null,
    detail: `resource line source is still pending ${status.operation} settlement`,
  });
}

function failedSettlement(status, freshness, visibleSelection) {
  return freezeSettlement({
    kind: "failed",
    operation: status.operation,
    failureKind: status.kind,
    continuity: status.continuity,
    confirmationKind: null,
    freshnessKind: freshness.kind,
    freshnessReason: freshness.kind === "stale" ? freshness.reason : null,
    visibleSelectionKind: visibleSelection.kind,
    branchProof: visibleSelection.branchProof,
    rebaseProof: visibleSelection.rebaseProof,
    message: status.kind === "rejected" ? status.message : null,
    retryRecommended: true,
    retryOperation: status.operation,
    detail: status.kind === "rejected"
      ? `resource line source ${status.operation} settlement was rejected`
      : `resource line source ${status.operation} settlement timed out`,
  });
}

function confirmedSettlement(status, freshness, visibleSelection, mutationResponse) {
  return freezeSettlement({
    kind: "confirmed",
    operation: status.operation,
    failureKind: null,
    continuity: null,
    confirmationKind: mutationResponse.confirmationKind,
    freshnessKind: freshness.kind,
    freshnessReason: freshness.kind === "stale" ? freshness.reason : null,
    visibleSelectionKind: visibleSelection.kind,
    branchProof: visibleSelection.branchProof,
    rebaseProof: visibleSelection.rebaseProof,
    message: null,
    retryRecommended: false,
    retryOperation: null,
    detail: `resource line source exposes ${mutationResponse.confirmationKind} confirmation posture`,
  });
}

function noneSettlement(status, freshness, visibleSelection) {
  return freezeSettlement({
    kind: "none",
    operation: status.operation,
    failureKind: null,
    continuity: null,
    confirmationKind: null,
    freshnessKind: freshness.kind,
    freshnessReason: freshness.kind === "stale" ? freshness.reason : null,
    visibleSelectionKind: visibleSelection.kind,
    branchProof: visibleSelection.branchProof,
    rebaseProof: visibleSelection.rebaseProof,
    message: null,
    retryRecommended: false,
    retryOperation: null,
    detail: "resource line source does not carry confirmation or failure settlement proof",
  });
}

function freezeSettlement(settlement) {
  return Object.freeze({
    ...settlement,
    digest: stableValueDigest(settlement),
  });
}
