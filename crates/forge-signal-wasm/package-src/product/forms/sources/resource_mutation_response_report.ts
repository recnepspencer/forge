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
    unsupportedTargetDigest: summary.unsupportedTargetDigest,
    noHiddenMutationDigest: summary.noHiddenMutationDigest,
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
