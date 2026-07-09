import { readMutationResponseSummaryDigest } from "../../resource/mutation/resource_mutation_response_diagnostics_projection.js";
import { stableValueDigest } from "../values/value_paths.js";

export function readFormResourceMutationResponseReport(mutationResponse, planCount = 1) {
  if (mutationResponse === null) {
    return null;
  }
  const summary = readMutationResponseSummaryDigest({
    lastMutationResponsePlan: mutationResponse,
    mutationResponsePlanCount: planCount,
  });
  const contract = summarizeMutationResponseContract(summary);
  return Object.freeze({
    confirmationKind: summary.confirmationKind,
    confirmationDigest: summary.confirmationDigest,
    targetCount: summary.targetCount,
    exactTargetCount: summary.exactTargetCount,
    fallbackTargetCount: summary.fallbackTargetCount,
    freshnessPostureDigest: summary.freshnessPostureDigest,
    fallbackReasonDigest: summary.fallbackReasonDigest,
    fallbackAffectedTargetDigest: summary.fallbackAffectedTargetDigest,
    staleTargetReasonDigest: summary.staleTargetReasonDigest,
    staleTargetAffectedTargetDigest: summary.staleTargetAffectedTargetDigest,
    deliveryAwaitedDigest: summary.deliveryAwaitedDigest,
    refetchRequiredDigest: summary.refetchRequiredDigest,
    partialReconciliationDigest: summary.partialReconciliationDigest,
    outOfContractTargetDigest: summary.unsupportedTargetDigest,
    noHiddenMutationDigest: summary.noHiddenMutationDigest,
    contract,
    targetOutcomeDigest: summary.targetOutcomeDigest,
    targetOutcomes: summary.targetOutcomes,
    replayExactDigest: summary.replayExactDigest,
    restoreExactDigest: summary.restoreExactDigest,
    rollbackDigest: summary.rollbackDigest,
    mergeRebaseDigest: summary.mergeRebaseDigest,
    executionDigest: summary.executionDigest,
    diagnosticCount: summary.diagnosticCount,
    diagnosticDigest: summary.diagnosticDigest,
    planCount: summary.planCount,
    completion: summarizeCompletion(summary.targetOutcomes, mutationResponse.targets ?? []),
    identityMigration: summary.identityMigrationDigest === undefined
      ? null
      : Object.freeze({
        digest: summary.identityMigrationDigest,
        needed: summary.identityMigrationNeeded,
        partialAdmission: summary.identityMigrationPartialAdmission,
        targetCount: summary.identityMigrationTargetCount,
        exactTargetCount: summary.identityMigrationExactTargetCount,
        executionDigest: summary.identityMigrationExecutionDigest,
        fallbackDigest: summary.identityMigrationFallbackDigest,
      }),
    digest: stableValueDigest(summary),
  });
}

function summarizeMutationResponseContract(summary) {
  const contract = {
    deliveryAwaitedDigest: summary.deliveryAwaitedDigest,
    refetchRequiredDigest: summary.refetchRequiredDigest,
    partialReconciliationDigest: summary.partialReconciliationDigest,
    outOfContractTargetDigest: summary.unsupportedTargetDigest,
  };
  return Object.freeze({
    ...contract,
    digest: stableValueDigest(contract),
  });
}

function summarizeCompletion(targetOutcomes, targets) {
  const familyKinds = [...new Set(targetOutcomes.map((outcome) => outcome.familyKind))].sort();
  const placementKinds = targets
    .filter((target) => target.execution.kind === "exactCollectionInsert")
    .map((target) => target.execution.placement)
    .filter((placement) => placement === "append" || placement === "prepend");
  const deleteKinds = targetOutcomes
    .map((outcome) => outcome.executionKind === "exactCollectionDelete"
      ? "delete"
      : outcome.executionKind === "exactCollectionTombstone"
        ? "tombstone"
        : null)
    .filter((kind) => kind !== null);
  const placementKind = placementKinds.length === 0
    ? "none"
    : placementKinds.every((placement) => placement === "append")
      ? "appendOnly"
      : placementKinds.every((placement) => placement === "prepend")
        ? "prependOnly"
        : "mixed";
  const deletionKind = deleteKinds.length === 0
    ? "none"
    : deleteKinds.every((kind) => kind === "delete")
      ? "deleteOnly"
      : deleteKinds.every((kind) => kind === "tombstone")
        ? "tombstoneOnly"
        : "mixed";
  const summary = {
    multiFamily: familyKinds.length > 1,
    familyKinds: Object.freeze(familyKinds),
    exactTargetCount: targetOutcomes.filter((outcome) => outcome.outcomeKind === "exact").length,
    fallbackTargetCount: targetOutcomes.filter((outcome) => outcome.outcomeKind === "fallback").length,
    familyCounts: Object.freeze({
      detail: targetOutcomes.filter((outcome) => outcome.familyKind === "detail").length,
      collection: targetOutcomes.filter((outcome) => outcome.familyKind === "collection").length,
      paged: targetOutcomes.filter((outcome) => outcome.familyKind === "paged").length,
    }),
    placement: Object.freeze({
      kind: placementKind,
      count: placementKinds.length,
      appendCount: placementKinds.filter((placement) => placement === "append").length,
      prependCount: placementKinds.filter((placement) => placement === "prepend").length,
    }),
    deletion: Object.freeze({
      kind: deletionKind,
      count: deleteKinds.length,
      deleteCount: deleteKinds.filter((kind) => kind === "delete").length,
      tombstoneCount: deleteKinds.filter((kind) => kind === "tombstone").length,
    }),
    summaryTargetCount: targetOutcomes.filter((outcome) => outcome.executionKind === "exactSummary").length,
  };
  return Object.freeze({
    ...summary,
    digest: stableValueDigest(summary),
  });
}
