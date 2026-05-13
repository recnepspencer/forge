function readMutationResponsePlanRecord(diagnostics) {
  if (!("lastMutationResponsePlan" in diagnostics)) {
    return null;
  }
  return Object.freeze({
    plan: diagnostics.lastMutationResponsePlan,
    planCount: diagnostics.mutationResponsePlanCount,
  });
}

function readMutationResponseSummaryDigest(diagnostics) {
  const mutationResponsePlanRecord = readMutationResponsePlanRecord(diagnostics);
  if (mutationResponsePlanRecord === null) {
    return null;
  }
  const identityMigration = mutationResponsePlanRecord.plan.identityMigration;
  return Object.freeze({
    planId: mutationResponsePlanRecord.plan.planId,
    targetCount: mutationResponsePlanRecord.plan.targetCount,
    confirmationKind: mutationResponsePlanRecord.plan.confirmation.kind,
    confirmationDigest: mutationResponsePlanRecord.plan.confirmation.digest,
    rollbackDigest: mutationResponsePlanRecord.plan.lifecycleProof.rollbackDigest,
    mergeRebaseDigest:
      mutationResponsePlanRecord.plan.lifecycleProof.mergeRebaseDigest,
    executionDigest: mutationResponsePlanRecord.plan.executionDigest,
    diagnosticCount: mutationResponsePlanRecord.plan.diagnostics.count,
    diagnosticDigest: mutationResponsePlanRecord.plan.diagnostics.digest,
    planCount: mutationResponsePlanRecord.planCount,
    ...(identityMigration === null
      ? {}
      : {
          identityMigrationDigest: identityMigration.digest,
          identityMigrationNeeded: identityMigration.migrationNeeded,
          identityMigrationPartialAdmission: identityMigration.partialAdmission,
          identityMigrationTargetCount: identityMigration.targetCount,
          identityMigrationExactTargetCount: identityMigration.exactTargetCount,
          identityMigrationExecutionDigest: identityMigration.executionDigest,
          identityMigrationFallbackDigest: identityMigration.fallbackDigest,
        }),
  });
}

export {
  readMutationResponsePlanRecord,
  readMutationResponseSummaryDigest,
};
