import {
  createCommittedVisibleSelection,
  createHistory,
  createRequest,
  createVerificationPackage,
} from "./resource_line_fixture_shared.mjs";

export function createReadOnlyResourceLineFixture({
  effectProfile = null,
  status,
  freshness,
  visibleSelection = createCommittedVisibleSelection("resource line is showing committed server truth"),
  replayExactResult = null,
  restoreExactResult = null,
  mutationResponse = null,
  compatibility = null,
  familyKind = "detail",
  familyId = "task",
  runtimeLineId = "task:t1",
  canonicalKey = "id=t1",
}) {
  let currentStatus = status;
  let currentVisibleSelection = visibleSelection;
  let refreshCount = 0;
  let revalidateCount = 0;
  const request = createRequest(effectProfile, { familyKind, familyId, canonicalKey });
  return Object.freeze({
    effects: () => Object.freeze({ open: () => Object.freeze([]) }),
    value: () => ({ title: "Resource task" }),
    descriptor: () => ({
      family: { kind: familyKind, familyId },
      canonicalParams: { params: { id: "t1" }, canonicalKey },
      runtimeLineId,
      scopeId: "workspace",
      ...(compatibility === null ? {} : { compatibility }),
    }),
    request: () => request,
    summary: () => ({
      current: {
        status: currentStatus,
        freshness,
        hasVisibleValue: true,
        visibleValueVersion: 1,
        visibleSelection: currentVisibleSelection,
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
          status: currentStatus,
          freshness,
          hasVisibleValue: true,
          visibleValueVersion: 1,
          visibleSelection: currentVisibleSelection,
        },
        activity: {
          lastOperation: currentStatus.operation,
          lastOutcome: currentStatus.kind === "rejected" ? "rejected" : currentStatus.kind,
          pendingOperation: currentStatus.kind === "pending" ? currentStatus.operation : null,
          continuity: "preserveVisibleValue",
          freshnessPolicy: "stable",
        },
        counts: {
          refreshCount,
          revalidateCount,
          retryAttemptCount: 0,
          rejectionCount: currentStatus.kind === "rejected" ? 1 : 0,
          timeoutCount: currentStatus.kind === "timedOut" ? 1 : 0,
          supersessionCount: 0,
          invalidationCount: 0,
          patchCount: 0,
          deliveryCount: 0,
          basisAdvanceCount: 0,
        },
        latest: {
          basisCurrentId: "basis-1",
          effect: null,
          errorMessage: currentStatus.kind === "rejected" ? currentStatus.message : null,
        },
        request: { method: request.method, effects: effectProfile },
        processing: { kind: "ready", completionKind: "none", jobId: null, message: null },
        upload: { kind: "ready", transportKind: "none", uploadId: null, descriptor: null, finalizeRequired: false, awaitingProcessing: false, message: null },
        download: { count: 0, readyCount: 0, unavailableCount: 0, incompatibleCount: 0, descriptors: [] },
        explainability: { available: false, reason: "not requested" },
      },
      explainability: { available: false, reason: "not requested" },
    }),
    status: () => currentStatus,
    freshness: () => freshness,
    mutationResponse: () => mutationResponse,
    refresh() {
      refreshCount += 1;
      currentStatus = Object.freeze({ kind: "pending", operation: "refresh", continuity: "preserveVisibleValue" });
      return currentStatus;
    },
    revalidate() {
      revalidateCount += 1;
      currentStatus = Object.freeze({ kind: "pending", operation: "revalidate", continuity: "preserveVisibleValue" });
      return currentStatus;
    },
    history() {
      return Object.freeze({
        ...createHistory(createVerificationPackage({
          request,
          status: currentStatus,
          freshness,
          visibleSelection: currentVisibleSelection,
          patchCount: 0,
          lastEffect: null,
          mutationResponse,
          externalCompatibility: compatibility ?? Object.freeze({ kind: "native" }),
        })),
        replayExact() {
          if (replayExactResult !== null) return Object.freeze({ ...replayExactResult });
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
          if (restoreExactResult !== null) return Object.freeze({ ...restoreExactResult });
          currentStatus = Object.freeze({ kind: "fulfilled", operation: "restore" });
          currentVisibleSelection = Object.freeze({
            kind: "restored",
            source: "exactBranchRestore",
            effectId: null,
            branchId: 7,
            snapshotId: 11,
            basisId: "basis-1",
            rollbackKind: null,
            detail: "resource line visible truth was restored through exact line history restore",
          });
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
      });
    },
  });
}
