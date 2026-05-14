import { normalizeForProof } from "./resource_verification_package_helpers.mjs";

function createDeferred() {
  let resolve;
  const promise = new Promise((nextResolve) => {
    resolve = nextResolve;
  });
  return { promise, resolve };
}

async function settleRuntime() {
  await Promise.resolve();
  await new Promise((resolve) => setTimeout(resolve, 0));
}

function projectMutationCloseoutSnapshot(lines) {
  return {
    ...(lines.createLine ? { create: projectWriteLineDigest(lines.createLine) } : {}),
    ...(lines.collectionUpdateLine
      ? { collectionUpdate: projectWriteLineDigest(lines.collectionUpdateLine) }
      : {}),
    ...(lines.exactSaveLine
      ? { exactSave: projectWriteLineDigest(lines.exactSaveLine) }
      : {}),
    ...(lines.partialSaveLine
      ? { partialSave: projectWriteLineDigest(lines.partialSaveLine) }
      : {}),
    ...(lines.deliveryFallbackLine
      ? { deliveryFallback: projectWriteLineDigest(lines.deliveryFallbackLine) }
      : {}),
    ...(lines.staleSaveLine
      ? { staleSave: projectWriteLineDigest(lines.staleSaveLine) }
      : {}),
    ...(lines.removeLine ? { remove: projectWriteLineDigest(lines.removeLine) } : {}),
    ...(lines.duplicateRemoveLine
      ? { duplicateRemove: projectWriteLineDigest(lines.duplicateRemoveLine) }
      : {}),
    taskList: projectReadLineDigest(lines.taskListLine),
    taskCounts: projectReadLineDigest(lines.taskCountsLine),
    taskDetail: projectReadLineDigest(lines.taskDetailLine),
    permissions: projectReadLineDigest(lines.permissionsLine),
    staleDetail: projectReadLineDigest(lines.staleDetailLine),
  };
}

function projectWriteLineDigest(line) {
  const summaryLatest = line.summary().diagnostics.latest;
  const verification = line.history().verificationPackage();
  return {
    confirmationKind: line.mutationResponse().confirmation.kind,
    targetDigest: line.mutationResponse().targetDigest,
    fallbackDigest: line.mutationResponse().fallbackDigest,
    replayExactDigest: line.mutationResponse().lifecycleProof.replayExactDigest,
    restoreExactDigest: line.mutationResponse().lifecycleProof.restoreExactDigest,
    rollbackDigest: line.mutationResponse().lifecycleProof.rollbackDigest,
    mergeRebaseDigest: line.mutationResponse().lifecycleProof.mergeRebaseDigest,
    targetOutcomeDigest: summaryLatest.mutationResponseTargetOutcomeDigest ?? null,
    fallbackReasonDigest: summaryLatest.mutationResponseFallbackReasonDigest ?? null,
    staleTargetReasonDigest:
      summaryLatest.mutationResponseStaleTargetReasonDigest ?? null,
    staleTargetAffectedTargetDigest:
      summaryLatest.mutationResponseStaleTargetAffectedTargetDigest ?? null,
    freshnessPostureDigest:
      summaryLatest.mutationResponseFreshnessPostureDigest ?? null,
    deliveryAwaitedDigest:
      summaryLatest.mutationResponseDeliveryAwaitedDigest ?? null,
    refetchRequiredDigest:
      summaryLatest.mutationResponseRefetchRequiredDigest ?? null,
    noHiddenMutationDigest:
      summaryLatest.mutationResponseNoHiddenMutationDigest ?? null,
    summaryReplayExactDigest:
      summaryLatest.mutationResponseReplayExactDigest ?? null,
    summaryRestoreExactDigest:
      summaryLatest.mutationResponseRestoreExactDigest ?? null,
    identityMigrationDigest:
      summaryLatest.mutationResponseIdentityMigrationDigest ?? null,
    verificationPlanDigest: verification.mutationResponse?.plan.targetDigest ?? null,
    boundaryPerformanceEnvelope: normalizeForProof(
      verification.boundaryPerformanceEnvelope,
    ),
  };
}

function projectReadLineDigest(line) {
  const verification = line.history().verificationPackage();
  return {
    committedValue: normalizeForProof(verification.committedValue),
    restoreExact: normalizeForProof(
      verification.historyReplayRestore.availability.restoreExact,
    ),
    replayExact: normalizeForProof(
      verification.historyReplayRestore.availability.replayExact,
    ),
    typedReplayExact: normalizeForProof(verification.typedDenials.replayExact),
    typedRestoreExact: normalizeForProof(verification.typedDenials.restoreExact),
    latestLifecycleEvent: verification.historyReplayRestore.lastLifecycleEvent,
    mutationResponsePlanId:
      verification.diagnostics.summary.latest.mutationResponsePlanId ?? null,
    mutationResponseTargetOutcomeDigest:
      verification.diagnostics.summary.latest.mutationResponseTargetOutcomeDigest ?? null,
    mutationResponseFallbackReasonDigest:
      verification.diagnostics.summary.latest.mutationResponseFallbackReasonDigest ?? null,
    mutationResponseNoHiddenMutationDigest:
      verification.diagnostics.summary.latest.mutationResponseNoHiddenMutationDigest ?? null,
    boundaryPerformanceEnvelope: normalizeForProof(
      verification.boundaryPerformanceEnvelope,
    ),
  };
}

export { createDeferred, projectMutationCloseoutSnapshot, settleRuntime };
