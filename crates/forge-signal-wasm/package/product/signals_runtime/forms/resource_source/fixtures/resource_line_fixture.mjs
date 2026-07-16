import {
  createBranchAvailability,
  createCommittedVisibleSelection,
  createHistory,
  createMutationResponsePlanFixture,
  createRequest,
  createVerificationPackage,
} from "./resource_line_fixture_shared.mjs";

export { createMutationResponsePlanFixture };
export { createReadOnlyResourceLineFixture } from "./resource_line_read_only_fixture.mjs";

export function createDetailPatchLineFixture({
  effectProfile = null,
  initialValue,
  mutationResponse = null,
}) {
  let value = { ...initialValue };
  const baselineValue = { ...initialValue };
  let version = 1;
  let latestEffect = null;
  let currentStatus = Object.freeze({ kind: "fulfilled", operation: "initialLoad" });
  let restored = false;
  const patchHistory = [];
  const request = createRequest(effectProfile);
  const freshness = Object.freeze({ kind: "fresh" });
  const visibleSelection = () => latestEffect === null
    ? restored
      ? Object.freeze({
        kind: "restored",
        source: "exactBranchRestore",
        effectId: null,
        branchId: 7,
        snapshotId: 11,
        basisId: "basis-1",
        rollbackKind: null,
        detail: "resource line visible truth was restored through exact line history restore",
      })
      : createCommittedVisibleSelection("resource line is showing committed server truth")
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
    effects() {
      return Object.freeze({
        open: () => latestEffect === null
          ? Object.freeze([])
          : Object.freeze([Object.freeze({ effectId: "effect-1" })]),
      });
    },
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
        status: currentStatus,
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
          status: currentStatus,
          freshness,
          hasVisibleValue: true,
          visibleValueVersion: version,
          visibleSelection: visibleSelection(),
        },
        activity: {
          lastOperation: currentStatus.operation,
          lastOutcome: currentStatus.kind === "rejected" ? "rejected" : currentStatus.kind,
          pendingOperation: currentStatus.kind === "pending" ? currentStatus.operation : null,
          continuity: "preserveVisibleValue",
          freshnessPolicy: "stable",
        },
        counts: {
          refreshCount: 0,
          revalidateCount: 0,
          retryAttemptCount: 0,
          rejectionCount: currentStatus.kind === "rejected" ? 1 : 0,
          timeoutCount: currentStatus.kind === "timedOut" ? 1 : 0,
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
          errorMessage: currentStatus.kind === "rejected" ? currentStatus.message : null,
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
    status: () => currentStatus,
    freshness: () => freshness,
    mutationResponse: () => mutationResponse,
    history() {
      const verificationPackage = createVerificationPackage({
        request,
        status: currentStatus,
        freshness,
        visibleSelection: visibleSelection(),
        patchCount: patchHistory.length,
        lastEffect: latestEffect,
        mutationResponse,
      });
      return Object.freeze({
        ...createHistory(verificationPackage),
        replayExact() {
          currentStatus = Object.freeze({ kind: "fulfilled", operation: "replay" });
          return Object.freeze({
            kind: "replayed",
            mode: "SameRuntimeSignalExact",
            signalId: "task:t1",
            basisCurrentId: "basis-1",
            basisAdvanceCount: 0,
            reloadStatus: currentStatus,
          });
        },
        restoreExact() {
          value = { ...baselineValue };
          version += 1;
          latestEffect = null;
          restored = true;
          currentStatus = Object.freeze({ kind: "fulfilled", operation: "restore" });
          return Object.freeze({
            kind: "restored",
            mode: "SameRuntimeBranchExact",
            branchId: 7,
            snapshotId: 11,
            basisCurrentId: "basis-1",
            basisAdvanceCount: 0,
            reloadStatus: currentStatus,
          });
        },
        rollbackLastEffect() {
          if (latestEffect === null) {
            return Object.freeze({
              kind: "unavailable",
              reason: "noOpenEffect",
              detail: "resource effect rollback is unavailable because the line has no recorded resource effect",
              effectId: null,
              basisCurrentId: "basis-1",
              basisAdvanceCount: 0,
            });
          }
          value = { ...baselineValue };
          version += 1;
          latestEffect = null;
          currentStatus = Object.freeze({ kind: "fulfilled", operation: "restore" });
          return Object.freeze({
            kind: "rejectedAndRetired",
            effectId: "effect-1",
            canonicalValue: { ...baselineValue },
            retired: Object.freeze([Object.freeze({
              effectId: "effect-1",
              retirement: Object.freeze({ reason: "rejected" }),
            })]),
            projection: Object.freeze({
              kind: "canonical",
              projectedValue: { ...baselineValue },
              projectionDigest: "fixture:canonical-projection",
            }),
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
      restored = false;
      currentStatus = Object.freeze({ kind: "fulfilled", operation: "patch" });
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
