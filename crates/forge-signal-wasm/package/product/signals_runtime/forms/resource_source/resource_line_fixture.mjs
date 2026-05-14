function createRequest(effectProfile) {
  return Object.freeze({
    family: { kind: "detail", familyId: "task" },
    canonicalParams: { params: { id: "t1" }, canonicalKey: "id=t1" },
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

function createCommittedVisibleSelection(detail) {
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

function createBranchAvailability() {
  return Object.freeze({
    replay: { kind: "available" },
    replayExact: { kind: "available", mode: "SameRuntimeSignalExact", signalId: "task:t1" },
    lineage: { kind: "available" },
    branch: { kind: "available" },
    restoreExact: { kind: "available", mode: "SameRuntimeBranchExact", branchId: 7, snapshotId: 11 },
  });
}

function createBranchSummary() {
  return Object.freeze({
    id: 7,
    name: "task-speculative",
    parentBranchId: null,
    headSnapshotId: 11,
  });
}

function createVerificationPackage({
  request,
  status,
  freshness,
  visibleSelection,
  patchCount,
  lastEffect,
  mutationResponse,
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
    mutationResponse: mutationResponse === null
      ? undefined
      : { plan: mutationResponse, planCount: mutationResponse.planCount ?? 1 },
  });
}

function createHistory(verificationPackage) {
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
} = {}) {
  const execution = fallbackKind === null
    ? Object.freeze({ kind: "exactDetail", scope: "field", field: "title" })
    : Object.freeze({
      kind: "fallback",
      fallback: fallbackKind,
      partial: fallbackKind === "partialReconciliation"
        ? { kind: "missingResponseField", field: "title" }
        : null,
      staleness: staleReason === null ? null : { reason: staleReason },
    });
  return Object.freeze({
    planId: "mutation-plan-1",
    planCount,
    targetCount: 1,
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
    targets: Object.freeze([Object.freeze({
      targetId: "target-1",
      family: Object.freeze({ kind: "detail", familyId: "task" }),
      line: Object.freeze({ canonicalKey: "id=t1", residency: "resident" }),
      execution,
      targetDigest: "target:digest:1",
    })]),
  });
}

export function createReadOnlyResourceLineFixture({
  effectProfile = null,
  status,
  freshness,
  visibleSelection = createCommittedVisibleSelection("resource line is showing committed server truth"),
  mutationResponse = null,
}) {
  const request = createRequest(effectProfile);
  const verificationPackage = createVerificationPackage({
    request,
    status,
    freshness,
    visibleSelection,
    patchCount: 0,
    lastEffect: null,
    mutationResponse,
  });
  return Object.freeze({
    value: () => ({ title: "Resource task" }),
    descriptor: () => ({
      family: { kind: "detail", familyId: "task" },
      canonicalParams: { params: { id: "t1" }, canonicalKey: "id=t1" },
      runtimeLineId: "task:t1",
      scopeId: "workspace",
    }),
    request: () => request,
    summary: () => ({
      current: {
        status,
        freshness,
        hasVisibleValue: true,
        visibleValueVersion: 1,
        visibleSelection,
      },
      request,
      processing: { kind: "ready", completionKind: "none", jobId: null, message: null },
      upload: {
        kind: "ready",
        transportKind: "none",
        uploadId: null,
        descriptor: null,
        finalizeRequired: false,
        awaitingProcessing: false,
        message: null,
      },
      download: { count: 0, readyCount: 0, unavailableCount: 0, incompatibleCount: 0, descriptors: [] },
      diagnostics: {
        current: {
          status,
          freshness,
          hasVisibleValue: true,
          visibleValueVersion: 1,
          visibleSelection,
        },
        activity: {
          lastOperation: status.operation,
          lastOutcome: status.kind === "rejected" ? "rejected" : status.kind,
          pendingOperation: status.kind === "pending" ? status.operation : null,
          continuity: "preserveVisibleValue",
          freshnessPolicy: "stable",
        },
        counts: {
          refreshCount: 0,
          revalidateCount: 0,
          retryAttemptCount: 0,
          rejectionCount: status.kind === "rejected" ? 1 : 0,
          timeoutCount: status.kind === "timedOut" ? 1 : 0,
          supersessionCount: 0,
          invalidationCount: 0,
          patchCount: 0,
          deliveryCount: 0,
          basisAdvanceCount: 0,
        },
        latest: {
          basisCurrentId: "basis-1",
          effect: null,
          errorMessage: status.kind === "rejected" ? status.message : null,
        },
        request: {
          method: request.method,
          effects: effectProfile,
        },
        processing: { kind: "ready", completionKind: "none", jobId: null, message: null },
        upload: { kind: "ready", transportKind: "none", uploadId: null, descriptor: null, finalizeRequired: false, awaitingProcessing: false, message: null },
        download: { count: 0, readyCount: 0, unavailableCount: 0, incompatibleCount: 0, descriptors: [] },
        explainability: { available: false, reason: "not requested" },
      },
      explainability: { available: false, reason: "not requested" },
    }),
    status: () => status,
    freshness: () => freshness,
    mutationResponse: () => mutationResponse,
    history: () => createHistory(verificationPackage),
  });
}

export function createDetailPatchLineFixture({
  effectProfile = null,
  initialValue,
  mutationResponse = null,
}) {
  let value = { ...initialValue };
  const baselineValue = { ...initialValue };
  let version = 1;
  let latestEffect = null;
  const patchHistory = [];
  const request = createRequest(effectProfile);
  const status = Object.freeze({ kind: "fulfilled", operation: "initialLoad" });
  const freshness = Object.freeze({ kind: "fresh" });
  const visibleSelection = () => latestEffect === null
    ? createCommittedVisibleSelection("resource line is showing committed server truth")
    : Object.freeze({
      kind: "speculative",
      source: "localPatch",
      effectId: "effect-1",
      branchId: 7,
      snapshotId: 11,
      basisId: "basis-1",
      rollbackKind: latestEffect.optimistic.rollback.kind,
      detail: "resource line is showing speculative branch truth",
    });
  return Object.freeze({
    value: () => ({ ...value }),
    descriptor: () => ({
      family: { kind: "detail", familyId: "task" },
      canonicalParams: { params: { id: "t1" }, canonicalKey: "id=t1" },
      runtimeLineId: "task:t1",
      scopeId: "workspace",
    }),
    request: () => request,
    summary: () => ({
      current: {
        status,
        freshness,
        hasVisibleValue: true,
        visibleValueVersion: version,
        visibleSelection: visibleSelection(),
      },
      request,
      processing: { kind: "ready", completionKind: "none", jobId: null, message: null },
      upload: { kind: "ready", transportKind: "none", uploadId: null, descriptor: null, finalizeRequired: false, awaitingProcessing: false, message: null },
      download: { count: 0, readyCount: 0, unavailableCount: 0, incompatibleCount: 0, descriptors: [] },
      diagnostics: {
        current: {
          status,
          freshness,
          hasVisibleValue: true,
          visibleValueVersion: version,
          visibleSelection: visibleSelection(),
        },
        activity: {
          lastOperation: "delivery",
          lastOutcome: "fulfilled",
          pendingOperation: null,
          continuity: "preserveVisibleValue",
          freshnessPolicy: "stable",
        },
        counts: {
          refreshCount: 0,
          revalidateCount: 0,
          retryAttemptCount: 0,
          rejectionCount: 0,
          timeoutCount: 0,
          supersessionCount: 0,
          invalidationCount: 0,
          patchCount: patchHistory.length,
          deliveryCount: 0,
          basisAdvanceCount: 0,
        },
        latest: {
          patchKind: latestEffect?.patch?.kind ?? null,
          patchScope: latestEffect?.locus.kind ?? null,
          patchedField: latestEffect?.patch?.field ?? null,
          patchedRegion: latestEffect?.patch?.region ?? null,
          patchedPath: latestEffect?.patch?.path ?? null,
          basisCurrentId: "basis-1",
          effect: latestEffect,
          errorMessage: null,
        },
        request: {
          method: request.method,
          effects: effectProfile,
        },
        processing: { kind: "ready", completionKind: "none", jobId: null, message: null },
        upload: { kind: "ready", transportKind: "none", uploadId: null, descriptor: null, finalizeRequired: false, awaitingProcessing: false, message: null },
        download: { count: 0, readyCount: 0, unavailableCount: 0, incompatibleCount: 0, descriptors: [] },
        explainability: { available: false, reason: "not requested" },
      },
      explainability: { available: false, reason: "not requested" },
    }),
    diagnosticsSummary() {
      return this.summary().diagnostics;
    },
    status: () => status,
    freshness: () => freshness,
    mutationResponse: () => mutationResponse,
    history() {
      const verificationPackage = createVerificationPackage({
        request,
        status,
        freshness,
        visibleSelection: visibleSelection(),
        patchCount: patchHistory.length,
        lastEffect: latestEffect,
        mutationResponse,
      });
      return Object.freeze({
        ...createHistory(verificationPackage),
        rollbackLastEffect() {
          if (latestEffect === null) {
            return Object.freeze({
              kind: "unavailable",
              reason: "noEffect",
              detail: "resource effect rollback is unavailable because the line has no recorded resource effect",
              effectId: null,
              basisCurrentId: "basis-1",
              basisAdvanceCount: 0,
              rollback: null,
            });
          }
          value = { ...baselineValue };
          version += 1;
          const rollback = latestEffect.optimistic.rollback;
          latestEffect = null;
          return Object.freeze({
            kind: "rolledBack",
            mode: rollback.mode,
            effectId: "effect-1",
            branchId: rollback.branchId,
            snapshotId: rollback.snapshotId,
            basisCurrentId: "basis-1",
            basisAdvanceCount: 0,
            rollback,
            reloadStatus: status,
          });
        },
      });
    },
    reconciliation: () => ({
      broadReplace: true,
      narrowItem: false,
      narrowField: true,
      narrowRegion: false,
      narrowJsonPath: false,
      narrowSummary: false,
      fieldNames: ["title", "status"],
      regionNames: [],
      jsonPathNames: [],
      aspectNames: [],
      summaryNames: [],
    }),
    patch(patch) {
      if (patch.kind !== "field") {
        throw new TypeError("only detail field patches are supported in the test resource line");
      }
      patchHistory.push(patch);
      value = {
        ...value,
        [patch.field]: patch.value,
      };
      version += 1;
      latestEffect = {
        provenance: { kind: "speculative" },
        family: { kind: "detail", familyId: "task" },
        line: { runtimeLineId: "task:t1", scopeId: "workspace" },
        locus: { kind: "field", field: patch.field },
        patch,
        delivery: null,
        optimistic: {
          rollback: {
            kind: "compactInverseAvailable",
            mode: "CompactInversePatch",
            branchId: 7,
            snapshotId: 11,
            inverse: { patch },
            detail: "fixture rollback is available through the compact inverse patch",
          },
        },
      };
      return Object.freeze({
        kind: "narrowed",
        scope: "field",
        itemId: null,
        aspect: null,
        field: patch.field,
      });
    },
    patchHistory: () => patchHistory.slice(),
  });
}
