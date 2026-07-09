import {
  createCommittedVisibleSelection,
  createHistory,
  createRequest,
  createVerificationPackage,
} from "./resource_line_fixture_shared.mjs";

export function createCollectionPatchLineFixture({
  effectProfile = null,
  familyKind = "collection",
  familyId = "task-list",
  runtimeLineId = "task:list",
  canonicalKey = "workspace=current",
  initialItems,
}) {
  let value = { items: initialItems.map((item) => ({ ...item })) };
  let version = 1;
  let latestEffect = null;
  let currentStatus = Object.freeze({ kind: "fulfilled", operation: "initialLoad" });
  const patchHistory = [];
  const request = createRequest(effectProfile, { familyKind, familyId, canonicalKey });
  const freshness = Object.freeze({ kind: "fresh" });
  const visibleSelection = () => latestEffect === null
    ? createCommittedVisibleSelection("resource line is showing committed collection truth")
    : Object.freeze({
        kind: "speculative",
        source: "localPatch",
        effectId: `effect-${patchHistory.length}`,
        branchId: 7,
        snapshotId: 11,
        basisId: "basis-1",
        rollbackKind: latestEffect.optimistic.rollback.kind,
        detail: "resource line is showing speculative collection branch truth",
      });
  return Object.freeze({
    value: () => ({ items: value.items.map((item) => ({ ...item })) }),
    descriptor: () => ({
      family: { kind: familyKind, familyId },
      canonicalParams: { params: { scope: "current" }, canonicalKey },
      runtimeLineId,
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
          patchedField: null,
          patchedRegion: null,
          patchedPath: null,
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
    status: () => currentStatus,
    freshness: () => freshness,
    mutationResponse: () => null,
    history() {
      return Object.freeze({
        ...createHistory(createVerificationPackage({
          request,
          status: currentStatus,
          freshness,
          visibleSelection: visibleSelection(),
          patchCount: patchHistory.length,
          lastEffect: latestEffect,
          mutationResponse: null,
        })),
      });
    },
    reconciliation: () => ({
      broadReplace: true,
      narrowItem: true,
      narrowField: false,
      narrowRegion: false,
      narrowJsonPath: false,
      narrowSummary: false,
      fieldNames: [],
      regionNames: [],
      jsonPathNames: [],
      aspectNames: [],
      summaryNames: [],
    }),
    patch(patch) {
      patchHistory.push(patch);
      if (patch.kind === "replace") {
        value = { items: patch.nextValue.items.map((item) => ({ ...item })) };
      } else if (patch.kind === "item") {
        value = {
          items: value.items.map((item) => (item.id === patch.itemId ? { ...patch.nextItem } : item)),
        };
      } else if (patch.kind === "insert") {
        const nextItem = { ...patch.nextItem };
        value = {
          items: patch.placement === "prepend"
            ? [nextItem, ...value.items]
            : [...value.items, nextItem],
        };
      } else if (patch.kind === "delete") {
        value = {
          items: value.items.filter((item) => item.id !== patch.itemId),
        };
      } else {
        throw new TypeError(`unsupported collection fixture patch kind ${patch.kind}`);
      }
      version += 1;
      latestEffect = {
        provenance: { kind: "speculative" },
        family: { kind: familyKind, familyId },
        line: { runtimeLineId, scopeId: "workspace" },
        locus: { kind: patch.kind === "replace" ? "wholeForm" : "item", field: null, itemId: patch.itemId ?? null },
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
      currentStatus = Object.freeze({ kind: "fulfilled", operation: "patch" });
      return Object.freeze({
        kind: patch.kind === "replace" ? "replaced" : "narrowed",
        scope: patch.kind === "replace" ? "wholeForm" : "item",
        itemId: patch.itemId ?? null,
        aspect: null,
        field: null,
      });
    },
    patchHistory: () => patchHistory.slice(),
  });
}
