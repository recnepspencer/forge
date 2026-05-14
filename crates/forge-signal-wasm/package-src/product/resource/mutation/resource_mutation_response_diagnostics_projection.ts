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
  const targetOutcomes = readMutationResponseTargetOutcomes(
    mutationResponsePlanRecord.plan.targets,
  );
  const fallbackSummary = readMutationResponseFallbackSummary(
    targetOutcomes.entries,
    mutationResponsePlanRecord.plan.targetCount,
    mutationResponsePlanRecord.plan.confirmation.kind,
  );
  return Object.freeze({
    planId: mutationResponsePlanRecord.plan.planId,
    targetCount: mutationResponsePlanRecord.plan.targetCount,
    exactTargetCount: targetOutcomes.exactTargetCount,
    fallbackTargetCount: targetOutcomes.fallbackTargetCount,
    targetLookupBreadth:
      mutationResponsePlanRecord.plan.counters.targetLookupBreadth,
    targetFanoutBreadth:
      mutationResponsePlanRecord.plan.counters.targetFanoutBreadth,
    payloadFieldExtractionBreadth:
      mutationResponsePlanRecord.plan.counters.payloadFieldExtractionBreadth,
    topologyTraversalBreadth:
      mutationResponsePlanRecord.plan.counters.topologyTraversalBreadth,
    reconstructionBreadth:
      mutationResponsePlanRecord.plan.counters.reconstructionBreadth,
    fallbackBreadth: mutationResponsePlanRecord.plan.counters.fallbackBreadth,
    fallbackReasonDigest: fallbackSummary.reasonDigest,
    fallbackAffectedTargetDigest: fallbackSummary.affectedTargetDigest,
    staleTargetReasonDigest: fallbackSummary.staleTargetReasonDigest,
    staleTargetAffectedTargetDigest:
      fallbackSummary.staleTargetAffectedTargetDigest,
    freshnessPostureDigest: fallbackSummary.freshnessPostureDigest,
    deliveryAwaitedDigest: fallbackSummary.deliveryAwaitedDigest,
    refetchRequiredDigest: fallbackSummary.refetchRequiredDigest,
    partialReconciliationDigest: fallbackSummary.partialReconciliationDigest,
    unsupportedTargetDigest: fallbackSummary.unsupportedTargetDigest,
    noHiddenMutationDigest: fallbackSummary.noHiddenMutationDigest,
    targetOutcomeDigest: targetOutcomes.digest,
    targetOutcomes: targetOutcomes.entries,
    confirmationKind: mutationResponsePlanRecord.plan.confirmation.kind,
    confirmationDigest: mutationResponsePlanRecord.plan.confirmation.digest,
    replayExactDigest:
      mutationResponsePlanRecord.plan.lifecycleProof.replayExactDigest,
    restoreExactDigest:
      mutationResponsePlanRecord.plan.lifecycleProof.restoreExactDigest,
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

function readMutationResponseTargetOutcomes(targets) {
  const entries = Object.freeze(
    targets.map((target) => createMutationResponseTargetOutcomeEntry(target)),
  );
  const exactTargetCount = entries.filter((entry) => entry.outcomeKind === "exact").length;
  const fallbackTargetCount = entries.length - exactTargetCount;
  return Object.freeze({
    entries,
    exactTargetCount,
    fallbackTargetCount,
    digest: createMutationResponseTargetOutcomeDigest(entries),
  });
}

function createMutationResponseTargetOutcomeEntry(target) {
  const execution = target.execution;
  return Object.freeze({
    targetId: target.targetId,
    familyKind: target.family.kind,
    familyId: target.family.familyId,
    canonicalKey: target.line.canonicalKey,
    residency: target.line.residency,
    outcomeKind: execution.kind === "fallback" ? "fallback" : "exact",
    executionKind: execution.kind,
    scope: execution.kind === "fallback" ? null : execution.scope,
    fallbackKind: execution.kind === "fallback" ? execution.fallback : null,
    partialKind: execution.kind === "fallback" ? execution.partial?.kind ?? null : null,
    partialField: execution.kind === "fallback" ? execution.partial?.field ?? null : null,
    staleReason: execution.kind === "fallback" ? execution.staleness?.reason ?? null : null,
    locus:
      execution.kind === "fallback"
        ? null
        : execution.itemId
          ?? execution.summary
          ?? execution.field
          ?? execution.region
          ?? execution.path
          ?? "line",
    targetDigest: target.targetDigest,
  });
}

function createMutationResponseTargetOutcomeDigest(entries) {
  if (entries.length === 0) {
    return "mutation-response-target-outcomes|none";
  }
  return `mutation-response-target-outcomes|${entries.map((entry) =>
    [
      entry.targetId,
      entry.familyKind,
      entry.familyId,
      entry.canonicalKey,
      entry.residency,
      entry.outcomeKind,
      entry.executionKind,
      entry.scope ?? "none",
      entry.fallbackKind ?? "none",
      entry.partialKind ?? "none",
      entry.partialField ?? "none",
      entry.staleReason ?? "none",
      entry.locus ?? "none",
    ].join(":")).join(",")}`;
}

function readMutationResponseFallbackSummary(
  targetOutcomes,
  targetCount,
  confirmationKind,
) {
  const fallbackOutcomes = targetOutcomes.filter((entry) => entry.outcomeKind === "fallback");
  return Object.freeze({
    reasonDigest: createMutationResponseFallbackReasonDigest(fallbackOutcomes),
    affectedTargetDigest: createMutationResponseFallbackAffectedTargetDigest(
      fallbackOutcomes,
    ),
    staleTargetReasonDigest: createMutationResponseStaleTargetReasonDigest(
      fallbackOutcomes,
    ),
    staleTargetAffectedTargetDigest:
      createMutationResponseStaleTargetAffectedTargetDigest(fallbackOutcomes),
    freshnessPostureDigest: createMutationResponseFreshnessPostureDigest(
      confirmationKind,
      targetCount,
      targetOutcomes.length - fallbackOutcomes.length,
      fallbackOutcomes.length,
    ),
    deliveryAwaitedDigest: createMutationResponseFallbackKindTargetDigest(
      fallbackOutcomes,
      "deliveryAwaited",
    ),
    refetchRequiredDigest: createMutationResponseFallbackKindTargetDigest(
      fallbackOutcomes,
      "refetchRequired",
    ),
    partialReconciliationDigest: createMutationResponseFallbackKindTargetDigest(
      fallbackOutcomes,
      "partialReconciliation",
    ),
    unsupportedTargetDigest: createMutationResponseFallbackKindTargetDigest(
      fallbackOutcomes,
      "unsupportedTarget",
    ),
    noHiddenMutationDigest: createMutationResponseNoHiddenMutationDigest(
      targetCount,
      targetOutcomes.length - fallbackOutcomes.length,
      fallbackOutcomes.length,
    ),
  });
}

function createMutationResponseFallbackReasonDigest(fallbackOutcomes) {
  if (fallbackOutcomes.length === 0) {
    return "mutation-response-fallback-reasons|none";
  }
  const counts = new Map();
  for (const entry of fallbackOutcomes) {
    counts.set(entry.fallbackKind, (counts.get(entry.fallbackKind) ?? 0) + 1);
  }
  const reasons = [...counts.entries()]
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([kind, count]) => `${kind}:${count}`);
  return `mutation-response-fallback-reasons|${reasons.join(",")}`;
}

function createMutationResponseFallbackAffectedTargetDigest(fallbackOutcomes) {
  if (fallbackOutcomes.length === 0) {
    return "mutation-response-fallback-targets|none";
  }
  return `mutation-response-fallback-targets|${fallbackOutcomes.map((entry) =>
    [
      entry.targetId,
      entry.familyKind,
      entry.familyId,
      entry.canonicalKey,
      entry.fallbackKind,
    ].join(":")).join(",")}`;
}

function createMutationResponseFreshnessPostureDigest(
  confirmationKind,
  targetCount,
  exactTargetCount,
  fallbackTargetCount,
) {
  return [
    "mutation-response-freshness-posture",
    confirmationKind,
    `targets:${targetCount}`,
    `exact:${exactTargetCount}`,
    `fallback:${fallbackTargetCount}`,
  ].join("|");
}

function createMutationResponseStaleTargetReasonDigest(fallbackOutcomes) {
  const staleOutcomes = fallbackOutcomes.filter((entry) => entry.staleReason !== null);
  if (staleOutcomes.length === 0) {
    return "mutation-response-stale-target-reasons|none";
  }
  const counts = new Map();
  for (const entry of staleOutcomes) {
    counts.set(entry.staleReason, (counts.get(entry.staleReason) ?? 0) + 1);
  }
  const reasons = [...counts.entries()]
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([reason, count]) => `${reason}:${count}`);
  return `mutation-response-stale-target-reasons|${reasons.join(",")}`;
}

function createMutationResponseStaleTargetAffectedTargetDigest(fallbackOutcomes) {
  const staleOutcomes = fallbackOutcomes.filter((entry) => entry.staleReason !== null);
  if (staleOutcomes.length === 0) {
    return "mutation-response-stale-targets|none";
  }
  return `mutation-response-stale-targets|${staleOutcomes.map((entry) =>
    [
      entry.targetId,
      entry.familyKind,
      entry.familyId,
      entry.canonicalKey,
      entry.staleReason,
    ].join(":")).join(",")}`;
}

function createMutationResponseFallbackKindTargetDigest(fallbackOutcomes, fallbackKind) {
  const matchingOutcomes = fallbackOutcomes.filter((entry) => entry.fallbackKind === fallbackKind);
  if (matchingOutcomes.length === 0) {
    return `mutation-response-${fallbackKind}-targets|none`;
  }
  return `mutation-response-${fallbackKind}-targets|${matchingOutcomes.map((entry) =>
    [
      entry.targetId,
      entry.familyKind,
      entry.familyId,
      entry.canonicalKey,
      entry.partialKind ?? "none",
      entry.partialField ?? "none",
    ].join(":")).join(",")}`;
}

function createMutationResponseNoHiddenMutationDigest(
  targetCount,
  exactTargetCount,
  fallbackTargetCount,
) {
  const accountedTargetCount = exactTargetCount + fallbackTargetCount;
  const status = accountedTargetCount === targetCount ? "allDeclaredTargetsAccountedFor" : "mismatch";
  return [
    "mutation-response-no-hidden-mutation",
    status,
    `declared:${targetCount}`,
    `accounted:${accountedTargetCount}`,
    `exact:${exactTargetCount}`,
    `fallback:${fallbackTargetCount}`,
  ].join("|");
}

export {
  readMutationResponsePlanRecord,
  readMutationResponseSummaryDigest,
};
