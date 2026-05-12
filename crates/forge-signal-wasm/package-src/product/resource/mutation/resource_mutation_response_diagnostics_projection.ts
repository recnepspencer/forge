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
    executionDigest: mutationResponsePlanRecord.plan.executionDigest,
    planCount: mutationResponsePlanRecord.planCount,
  });
}

export {
  readMutationResponsePlanRecord,
  readMutationResponseSummaryDigest,
};
