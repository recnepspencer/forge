import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";

test("resource lines expose one canonical facade and rematerialize with stable family identity", async () => {
  const runtime = await createRealResourceTestRuntime({
    current_branch: undefined,
  });
  try {
    const { mod, resource } = runtime;
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
      load: ({ id }) => ({ id }),
    });

    const first = detail.line({ id: "history" });
    const firstDescriptor = first.descriptor();
    const firstSignal = first.signal();
    const firstSignalId = firstSignal.id;
    const firstHistory = first.history();

    assert.deepEqual(first.value(), { id: "history" });
    assert.deepEqual(first.status(), {
      kind: "fulfilled",
      operation: "initialLoad",
    });
    assert.deepEqual(first.freshness(), { kind: "fresh" });
    assert.deepEqual(first.download(), {
      count: 0,
      readyCount: 0,
      unavailableCount: 0,
      incompatibleCount: 0,
      descriptors: [],
    });
    assert.deepEqual(first.summary(), {
      current: {
        status: {
          kind: "fulfilled",
          operation: "initialLoad",
        },
        freshness: { kind: "fresh" },
        hasVisibleValue: true,
        visibleValueVersion: 1,
      },
      request: first.request(),
      processing: first.processing(),
      upload: first.upload(),
      download: first.download(),
      diagnostics: first.diagnosticsSummary(),
      explainability: first.history().availability,
    });
    assert.deepEqual(firstSignal(), { id: "history" });
    assert.equal(Array.isArray(firstHistory.replay.frames), true);
    assert.equal(firstHistory.replay.frames.length, 1);
    assert.equal(Array.isArray(firstHistory.lineage.events), true);
    assert.equal(firstHistory.lineage.events.length, 1);
    assert.equal(firstHistory.branch, null);
    assert.deepEqual(firstHistory.basis, {
      currentBasisId: null,
      advanceCount: 0,
      lastAdvanceFromId: null,
      lastAdvanceToId: null,
      advances: [],
    });
    assert.deepEqual(firstHistory.availability, {
      replay: { kind: "available" },
      replayExact: {
        kind: "unavailable",
        reason: "unsupportedByRuntime",
        detail:
          "resource line exact replay is unavailable because the Signals runtime does not expose replay_signal_by_id(...)",
      },
      lineage: { kind: "available" },
      branch: {
        kind: "unavailable",
        reason: "unsupportedByRuntime",
        detail:
          "resource line branch history is unavailable because the Signals runtime does not expose current_branch(...)",
      },
      restoreExact: {
        kind: "unavailable",
        reason: "unsupportedByRuntime",
        detail:
          "resource line exact branch restore is unavailable because the Signals runtime does not expose current_branch(...)",
      },
    });
    assert.equal(firstHistory.lifecycle.length, 1);
    assert.equal(firstHistory.lifecycle[0].event, "materialized");
    assert.deepEqual(firstHistory.lifecycle[0].status, {
      kind: "fulfilled",
      operation: "initialLoad",
    });
    assert.deepEqual(firstHistory.lifecycle[0].freshness, { kind: "fresh" });
    assert.equal(firstHistory.lifecycle[0].visibleValueVersion, 1);
    const verificationPackage = firstHistory.verificationPackage();
    assert.deepEqual(verificationPackage.declaration, {
      familyKind: "detail",
      familyId: firstDescriptor.family.familyId,
      canonicalKey: firstDescriptor.canonicalParams.canonicalKey,
      runtimeLineId: firstDescriptor.runtimeLineId,
      scopeId: firstDescriptor.scopeId,
    });
    assert.deepEqual(verificationPackage.committedValue, { id: "history" });
    assert.deepEqual(verificationPackage.requestPosture, {
      authKind: "anonymous",
      headerNames: [],
      correlationId: null,
      branchId: null,
      basisId: null,
      continuationKind: "none",
      processingKind: "none",
      uploadKind: "none",
      effectsName: null,
    });
    assert.deepEqual(verificationPackage.processing, {
      kind: "ready",
      completionKind: "none",
      jobId: null,
      message: null,
    });
    assert.deepEqual(verificationPackage.upload, {
      kind: "ready",
      transportKind: "none",
      uploadId: null,
      finalizeRequired: false,
      awaitingProcessing: false,
      message: null,
      hasDescriptor: false,
    });
    assert.deepEqual(verificationPackage.lifecycle, {
      status: { kind: "fulfilled", operation: "initialLoad" },
      freshness: { kind: "fresh" },
      lastOperation: "initialLoad",
      lastOutcome: "fulfilled",
      pendingOperation: null,
      visibleValueVersion: 1,
      refreshCount: 0,
      revalidateCount: 0,
      retryAttemptCount: 0,
      rejectionCount: 0,
      timeoutCount: 0,
      supersessionCount: 0,
      invalidationCount: 0,
      patchCount: 0,
      deliveryCount: 0,
      basisAdvanceCount: 0,
      lastEffect: null,
    });
    assert.deepEqual(verificationPackage.continuity, {
      continuity: "preserveVisibleValue",
      hasVisibleValue: true,
      visibleValueVersion: 1,
    });
    assert.deepEqual(verificationPackage.reconciliation, {
      broadReplace: true,
      narrowItem: false,
      narrowField: false,
      narrowRegion: false,
      narrowJsonPath: false,
      narrowSummary: false,
      fieldNames: [],
      regionNames: [],
      jsonPathNames: [],
      aspectNames: [],
      summaryNames: [],
      lastPatchKind: null,
      lastPatchScope: null,
      lastPatchedItemId: null,
      lastPatchedField: null,
      lastPatchedRegion: null,
      lastPatchedPath: null,
      lastPatchedAspect: null,
      lastPatchedSummary: null,
    });
    assert.deepEqual(verificationPackage.diagnostics, {
      lastOperation: "initialLoad",
      lastOutcome: "fulfilled",
      pendingOperation: null,
      lastErrorMessage: null,
      summary: {
        current: {
          status: { kind: "fulfilled", operation: "initialLoad" },
          freshness: { kind: "fresh" },
          hasVisibleValue: true,
          visibleValueVersion: 1,
        },
        activity: {
          lastOperation: "initialLoad",
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
          patchCount: 0,
          deliveryCount: 0,
          basisAdvanceCount: 0,
        },
        latest: {
          invalidationCause: null,
          invalidationScope: null,
          patchKind: null,
          patchScope: null,
          patchedItemId: null,
          patchedField: null,
          patchedRegion: null,
          patchedPath: null,
          patchedAspect: null,
          patchedSummary: null,
          deliveryKind: null,
          deliveryScope: null,
          deliveryPacketId: null,
          deliveryBasisId: null,
          basisCurrentId: null,
          basisAdvanceFromId: null,
          basisAdvanceToId: null,
          effect: null,
          supersededOperation: null,
          timeoutOperation: null,
          errorMessage: null,
          preservedVisibleValueOnLastRejection: false,
        },
      },
    });
    assert.equal(Array.isArray(verificationPackage.historyReplayRestore.replay.frames), true);
    assert.equal(verificationPackage.historyReplayRestore.replay.frames.length, 1);
    assert.equal(Array.isArray(verificationPackage.historyReplayRestore.lineage.events), true);
    assert.equal(verificationPackage.historyReplayRestore.lineage.events.length, 1);
    assert.deepEqual(verificationPackage.historyReplayRestore.branch, null);
    assert.deepEqual(verificationPackage.historyReplayRestore.basis, firstHistory.basis);
    assert.deepEqual(verificationPackage.historyReplayRestore.availability, firstHistory.availability);
    assert.equal(verificationPackage.historyReplayRestore.lifecycleLength, 1);
    assert.equal(verificationPackage.historyReplayRestore.lastLifecycleEvent, "materialized");
    assert.deepEqual(verificationPackage.binaryDownload, {
      count: 0,
      readyCount: 0,
      unavailableCount: 0,
      incompatibleCount: 0,
      descriptorKinds: [],
    });
    assert.deepEqual(verificationPackage.deliveryProvenance, {
      deliveryCount: 0,
      lastDeliveryKind: null,
      lastDeliveryScope: null,
      lastDeliveryPacketId: null,
      lastDeliveryBasisId: null,
      lastEffect: null,
      basisCurrentId: null,
      basisAdvanceCount: 0,
      basisAdvanceFromId: null,
      basisAdvanceToId: null,
    });
    assert.deepEqual(verificationPackage.externalCompatibility, {
      kind: "native",
    });
    assert.deepEqual(verificationPackage.boundaryPerformanceEnvelope, {
      lifecycleEntryCount: 1,
      downloadDescriptorCount: 0,
      summaryReadShape: "inspectionSummary",
      commonLineReadShape: "groupedLineSummary",
    });
    assert.deepEqual(verificationPackage.capabilities, {
      summary: true,
      diagnosticsSummary: true,
      requestRead: true,
      processingRead: true,
      uploadRead: true,
      downloadRead: true,
      historyRead: true,
      patch: true,
      deliver: true,
      reconciliationRead: true,
      broadReplace: true,
      narrowItem: false,
      narrowField: false,
      narrowRegion: false,
      narrowJsonPath: false,
      narrowSummary: false,
    });
    assert.deepEqual(verificationPackage.typedDenials, {
      replay: null,
      replayExact: firstHistory.availability.replayExact,
      lineage: null,
      branch: firstHistory.availability.branch,
      restoreExact: firstHistory.availability.restoreExact,
    });

    first.free();
    const second = detail.line({ id: "history" });
    const secondDescriptor = second.descriptor();

    assert.equal(secondDescriptor.family.familyId, firstDescriptor.family.familyId);
    assert.equal(
      secondDescriptor.canonicalParams.canonicalKey,
      firstDescriptor.canonicalParams.canonicalKey,
    );
    assert.notEqual(secondDescriptor.runtimeLineId, firstDescriptor.runtimeLineId);
    assert.notEqual(second.signal().id, firstSignalId);
  } finally {
    await runtime.cleanup();
  }
});

test("released resource lines deny operational reads after free and Symbol.dispose", async () => {
  const runtime = await createRealResourceTestRuntime({
    current_branch: undefined,
  });
  try {
    const { mod, resource } = runtime;
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
      load: ({ id }) => ({ id }),
    });

    const line = detail.line({ id: "released" });
    line.free();

    const releasedMethods = [
      () => line.value(),
      () => line.signal(),
      () => line.request(),
      () => line.summary(),
      () => line.download(),
      () => line.history(),
      () => line.processing(),
      () => line.upload(),
      () => line.diagnostics(),
      () => line.diagnosticsSummary(),
      () => line.invalidate(),
      () => line.refresh(),
      () => line.revalidate(),
      () => line.status(),
      () => line.freshness(),
      () => line.view((value) => value?.id ?? null),
    ];

    for (const invoke of releasedMethods) {
      assert.throws(invoke, /cannot be used after line\.free\(\)/);
    }

    const disposed = detail.line({ id: "disposed" });
    disposed[Symbol.dispose]();
    assert.throws(
      () => disposed.value(),
      /cannot be used after line\.free\(\)/,
    );
  } finally {
    await runtime.cleanup();
  }
});
