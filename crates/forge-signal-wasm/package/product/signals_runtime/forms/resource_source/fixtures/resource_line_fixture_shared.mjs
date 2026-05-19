export function createRequest(effectProfile, options = {}) {
  const familyKind = options.familyKind ?? "detail";
  const familyId = options.familyId ?? "task";
  const canonicalKey = options.canonicalKey ?? "id=t1";
  return Object.freeze({
    family: { kind: familyKind, familyId },
    canonicalParams: { params: { id: "t1" }, canonicalKey },
    target: { baseUrl: null, requestPath: "/tasks/t1", url: "/tasks/t1" },
    baseUrl: null,
    method: "GET",
    body: null,
    auth: { kind: "anonymous" },
    context: { headers: {}, correlationId: null, branchId: null, basisId: null },
    continuation: { kind: "none" },
    processingJob: { kind: "none" },
    uploadTransport: { kind: "none" },
    effects: effectProfile,
  });
}

export function createCommittedVisibleSelection(detail) {
  return Object.freeze({
    kind: "committed",
    source: "initialLoad",
    effectId: null,
    branchId: null,
    snapshotId: null,
    basisId: "basis-1",
    detail,
  });
}

export function createBranchAvailability() {
  return Object.freeze({
    replay: { kind: "available" },
    replayExact: { kind: "available", mode: "SameRuntimeSignalExact", signalId: "task:t1" },
    lineage: { kind: "available" },
    branch: { kind: "available" },
    restoreExact: { kind: "available", mode: "SameRuntimeBranchExact", branchId: 7, snapshotId: 11 },
  });
}

export function createBranchSummary() {
  return Object.freeze({
    id: 7,
    name: "task-speculative",
    parentBranchId: null,
    headSnapshotId: 11,
  });
}

export function createVerificationPackage({
  request,
  status,
  freshness,
  visibleSelection,
  patchCount,
  lastEffect,
  mutationResponse,
  externalCompatibility = Object.freeze({ kind: "native" }),
}) {
  return Object.freeze({
    requestPosture: {
      method: request.method,
      effects: request.effects,
    },
    lifecycle: {
      status,
      freshness,
      patchCount,
      lastEffect,
      visibleSelection,
    },
    historyReplayRestore: {
      branch: createBranchSummary(),
      availability: createBranchAvailability(),
    },
    externalCompatibility,
    mutationResponse: mutationResponse === null
      ? undefined
      : { plan: mutationResponse, planCount: mutationResponse.planCount ?? 1 },
  });
}

export function createHistory(verificationPackage) {
  return Object.freeze({
    replay: null,
    lineage: null,
    branch: createBranchSummary(),
    basis: { currentId: "basis-1", advances: [] },
    availability: createBranchAvailability(),
    lifecycle: [],
    verificationPackage() {
      return verificationPackage;
    },
    rollbackLastEffect() {
      return Object.freeze({
        kind: "unavailable",
        reason: "noEffect",
        detail: "resource effect rollback is unavailable because the line has no recorded resource effect",
        effectId: null,
        basisCurrentId: "basis-1",
        basisAdvanceCount: 0,
        rollback: null,
      });
    },
  });
}

export function createMutationResponsePlanFixture({
  confirmationKind = "consumedCanonicalTruth",
  fallbackKind = null,
  staleReason = null,
  planCount = 1,
  targets = null,
} = {}) {
  const defaultTargets = targets ?? [Object.freeze({
    targetId: "target-1",
    family: Object.freeze({ kind: "detail", familyId: "task" }),
    line: Object.freeze({ canonicalKey: "id=t1", residency: "resident" }),
    execution: fallbackKind === null
      ? Object.freeze({ kind: "exactDetail", scope: "field", field: "title" })
      : Object.freeze({
        kind: "fallback",
        fallback: fallbackKind,
        partial: fallbackKind === "partialReconciliation"
          ? { kind: "missingResponseField", field: "title" }
          : null,
        staleness: staleReason === null ? null : { reason: staleReason },
      }),
    targetDigest: "target:digest:1",
  })];
  return Object.freeze({
    planId: "mutation-plan-1",
    planCount,
    targetCount: defaultTargets.length,
    confirmation: Object.freeze({ kind: confirmationKind, digest: `confirmation:${confirmationKind}` }),
    lifecycleProof: Object.freeze({
      replayExactDigest: "replay:exact",
      restoreExactDigest: "restore:exact",
      rollbackDigest: "rollback:available",
      mergeRebaseDigest: "merge:rebase",
      count: 1,
    }),
    diagnostics: Object.freeze({ count: 0, digest: "diagnostics:none" }),
    counters: Object.freeze({
      targetLookupBreadth: 1,
      targetFanoutBreadth: 1,
      payloadFieldExtractionBreadth: 1,
      topologyTraversalBreadth: 1,
      reconstructionBreadth: fallbackKind === null ? 1 : 0,
      fallbackBreadth: fallbackKind === null ? 0 : 1,
    }),
    executionDigest: `execution:${confirmationKind}:${fallbackKind ?? "exact"}`,
    identityMigration: null,
    targets: Object.freeze(defaultTargets),
  });
}
