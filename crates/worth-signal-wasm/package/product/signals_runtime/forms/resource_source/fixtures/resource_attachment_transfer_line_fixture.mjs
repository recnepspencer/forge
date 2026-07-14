import {
  createCommittedVisibleSelection,
  createHistory,
  createRequest,
  createVerificationPackage,
} from "./resource_line_fixture_shared.mjs";

export function createAttachmentTransferLineFixture({
  value,
  upload,
  processing,
  download,
  effectProfile = null,
  fieldNames = [],
  regionNames = [],
}) {
  let currentValue = structuredClone(value);
  let currentStatus = Object.freeze({ kind: "fulfilled", operation: "initialLoad" });
  let latestEffect = null;
  let version = 1;
  const freshness = Object.freeze({ kind: "fresh" });
  const request = createRequest(effectProfile);
  return Object.freeze({
    value: () => structuredClone(currentValue),
    descriptor: () => ({
      family: { kind: "detail", familyId: "task" },
      canonicalParams: { params: { id: "t1" }, canonicalKey: "id=t1" },
      runtimeLineId: "task:t1",
      scopeId: "workspace",
    }),
    request: () => request,
    summary: () => createSummary({
      status: currentStatus,
      freshness,
      visibleSelection: visibleSelectionFor(latestEffect),
      request,
      upload,
      processing,
      download,
      version,
      latestEffect,
    }),
    diagnosticsSummary() {
      return this.summary().diagnostics;
    },
    status: () => currentStatus,
    freshness: () => freshness,
    mutationResponse: () => null,
    history() {
      return createHistory(createVerificationPackage({
        request,
        status: currentStatus,
        freshness,
        visibleSelection: visibleSelectionFor(latestEffect),
        patchCount: latestEffect === null ? 0 : 1,
        lastEffect: latestEffect,
        mutationResponse: null,
      }));
    },
    reconciliation: () => ({
      broadReplace: true,
      narrowItem: false,
      narrowField: fieldNames.length > 0,
      narrowRegion: regionNames.length > 0,
      narrowJsonPath: false,
      narrowSummary: false,
      fieldNames,
      regionNames,
      jsonPathNames: [],
      aspectNames: [],
      summaryNames: [],
    }),
    patch(patch) {
      if (patch.kind !== "field" && patch.kind !== "region") {
        throw new TypeError(`unsupported attachment transfer fixture patch kind ${patch.kind}`);
      }
      currentValue = patch.kind === "field"
        ? { ...currentValue, [patch.field]: patch.value }
        : { ...currentValue, [patch.region]: patch.value };
      version += 1;
      latestEffect = {
        provenance: { kind: "speculative" },
        family: { kind: "detail", familyId: "task" },
        line: { runtimeLineId: "task:t1", scopeId: "workspace" },
        locus: patch.kind === "field"
          ? { kind: "field", field: patch.field }
          : { kind: "region", region: patch.region },
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
        kind: "narrowed",
        scope: patch.kind,
        itemId: null,
        aspect: null,
        field: patch.kind === "field" ? patch.field : null,
      });
    },
    upload: () => upload,
    processing: () => processing,
    download: () => download,
  });
}

export function createDownloadDescriptors(id) {
  return Object.freeze({
    count: 1,
    readyCount: 1,
    unavailableCount: 0,
    incompatibleCount: 0,
    descriptors: Object.freeze([Object.freeze({
      kind: "file",
      id,
      label: "Audit",
      fileName: "audit.pdf",
      mediaType: "application/pdf",
      byteLength: 512,
      download: {
        kind: "ready",
        transportKind: "simple",
        url: "https://example.test/download",
        method: "GET",
        headers: {},
        fields: {},
        objectKey: null,
        expiresAt: null,
      },
    })]),
  });
}

function visibleSelectionFor(latestEffect) {
  return latestEffect === null
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
}

function createSummary({ status, freshness, visibleSelection, request, upload, processing, download, version, latestEffect }) {
  return Object.freeze({
    current: {
      status,
      freshness,
      hasVisibleValue: true,
      visibleValueVersion: version,
      visibleSelection,
    },
    request,
    processing,
    upload,
    download,
    diagnostics: {
      current: {
        status,
        freshness,
        hasVisibleValue: true,
        visibleValueVersion: version,
        visibleSelection,
      },
      activity: {
        lastOperation: status.operation,
        lastOutcome: status.kind,
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
        patchCount: latestEffect === null ? 0 : 1,
        deliveryCount: 0,
        basisAdvanceCount: 0,
      },
      latest: {
        basisCurrentId: "basis-1",
        effect: latestEffect,
        errorMessage: null,
      },
      request: {
        method: request.method,
        effects: request.effects,
      },
      processing,
      upload,
      download,
      explainability: { available: false, reason: "not requested" },
    },
    explainability: { available: false, reason: "not requested" },
  });
}
