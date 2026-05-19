import {
  createCommittedVisibleSelection,
  createHistory,
  createRequest,
  createVerificationPackage,
} from "./resource_line_fixture_shared.mjs";

export function createDeclaredLocusDetailLineFixture({
  effectProfile = null,
  initialValue,
  fieldNames = [],
  jsonPathNames = [],
  regionNames = [],
  aspectNames = [],
  summaryNames = [],
}) {
  let value = structuredClone(initialValue);
  let version = 1;
  let latestEffect = null;
  let currentStatus = Object.freeze({ kind: "fulfilled", operation: "initialLoad" });
  const patchHistory = [];
  const request = createRequest(effectProfile);
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
    value: () => structuredClone(value),
    descriptor: () => ({
      family: { kind: "detail", familyId: "task" },
      canonicalParams: { params: { id: "t1" }, canonicalKey: "id=t1" },
      runtimeLineId: "task:t1",
      scopeId: "workspace",
    }),
    request: () => request,
    summary: () => ({
      current: { status: currentStatus, freshness, hasVisibleValue: true, visibleValueVersion: version, visibleSelection: visibleSelection() },
      request,
      processing: { kind: "ready", completionKind: "none", jobId: null, message: null },
      upload: { kind: "ready", transportKind: "none", uploadId: null, descriptor: null, finalizeRequired: false, awaitingProcessing: false, message: null },
      download: { count: 0, readyCount: 0, unavailableCount: 0, incompatibleCount: 0, descriptors: [] },
      diagnostics: {
        current: { status: currentStatus, freshness, hasVisibleValue: true, visibleValueVersion: version, visibleSelection: visibleSelection() },
        activity: { lastOperation: currentStatus.operation, lastOutcome: currentStatus.kind, pendingOperation: null, continuity: "preserveVisibleValue", freshnessPolicy: "stable" },
        counts: { refreshCount: 0, revalidateCount: 0, retryAttemptCount: 0, rejectionCount: 0, timeoutCount: 0, supersessionCount: 0, invalidationCount: 0, patchCount: patchHistory.length, deliveryCount: 0, basisAdvanceCount: 0 },
        latest: { patchKind: latestEffect?.patch?.kind ?? null, patchScope: latestEffect?.locus.kind ?? null, patchedField: latestEffect?.patch?.field ?? null, patchedRegion: latestEffect?.patch?.region ?? null, patchedPath: latestEffect?.patch?.path ?? null, basisCurrentId: "basis-1", effect: latestEffect, errorMessage: null },
        request: { method: request.method, effects: effectProfile },
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
      narrowItem: false,
      narrowField: fieldNames.length > 0,
      narrowRegion: regionNames.length > 0,
      narrowJsonPath: jsonPathNames.length > 0,
      narrowSummary: false,
      fieldNames,
      regionNames,
      jsonPathNames,
      aspectNames,
      summaryNames,
    }),
    patch(patch) {
      patchHistory.push(patch);
      if (patch.kind === "replace") {
        value = structuredClone(patch.nextValue);
      } else if (patch.kind === "field") {
        value = { ...value, [patch.field]: patch.value };
      } else if (patch.kind === "jsonPath") {
        value = writeJsonPathValue(value, patch.path, patch.value);
      } else if (patch.kind === "region") {
        value = { ...value, [patch.region]: patch.value };
      } else if (patch.kind === "itemAspect") {
        value = { ...value, [patch.aspect]: patch.value };
      } else if (patch.kind === "summary") {
        value = { ...value, [patch.summary]: patch.value };
      } else {
        throw new TypeError(`unsupported declared locus fixture patch kind ${patch.kind}`);
      }
      version += 1;
      latestEffect = {
        provenance: { kind: "speculative" },
        family: { kind: "detail", familyId: "task" },
        line: { runtimeLineId: "task:t1", scopeId: "workspace" },
        locus: patch.kind === "replace"
          ? { kind: "wholeForm" }
          : patch.kind === "field"
          ? { kind: "field", field: patch.field }
          : patch.kind === "jsonPath"
            ? { kind: "jsonPath", path: patch.path }
            : patch.kind === "region"
              ? { kind: "region", region: patch.region }
              : patch.kind === "itemAspect"
                ? { kind: "itemAspect", itemId: "task-1", aspect: patch.aspect }
                : { kind: "summary", summary: patch.summary },
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
        scope: patch.kind === "replace" ? "wholeForm" : patch.kind === "field" ? "field" : patch.kind,
        itemId: patch.kind === "itemAspect" ? "task-1" : null,
        aspect: patch.kind === "itemAspect" ? patch.aspect : null,
        field: patch.kind === "field" ? patch.field : null,
      });
    },
  });
}

function writeJsonPathValue(currentValue, path, nextValue) {
  const segments = path.replace(/^\$\./, "").split(".");
  const clone = structuredClone(currentValue);
  let cursor = clone;
  for (let index = 0; index < segments.length - 1; index += 1) {
    const key = segments[index];
    cursor[key] = typeof cursor[key] === "object" && cursor[key] !== null ? { ...cursor[key] } : {};
    cursor = cursor[key];
  }
  cursor[segments.at(-1)] = nextValue;
  return clone;
}
