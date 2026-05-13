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
  });
}

export {
  readMutationResponsePlanRecord,
  readMutationResponseSummaryDigest,
};
