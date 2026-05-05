import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

test("resource lines expose one canonical facade and rematerialize with stable family identity", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
      load: ({ id }) => ({ id }),
    });

    const first = detail.line({ id: "history" });
    const firstDescriptor = first.descriptor();
    const firstSignal = first.signal();
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
    assert.deepEqual(firstSignal(), { id: "history" });
    assert.deepEqual(firstHistory, {
      replay: { id: firstSignal.id, family: "replay" },
      lineage: { id: firstSignal.id, family: "lineage" },
      branch: null,
      basis: {
        currentBasisId: null,
        advanceCount: 0,
        lastAdvanceFromId: null,
        lastAdvanceToId: null,
        advances: [],
      },
      availability: {
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
      },
      lifecycle: [{
        sequence: 1,
        event: "materialized",
        status: {
          kind: "fulfilled",
          operation: "initialLoad",
        },
        freshness: {
          kind: "fresh",
        },
        lastOperation: "initialLoad",
        lastOutcome: "fulfilled",
        pendingOperation: null,
        statusContinuity: null,
        retryAttemptCount: 0,
        rejectionCount: 0,
        timeoutCount: 0,
        supersessionCount: 0,
        invalidationCount: 0,
        patchCount: 0,
        deliveryCount: 0,
        lastSupersededOperation: null,
        lastInvalidationCause: null,
        lastInvalidationScope: null,
        lastPatchKind: null,
        lastPatchScope: null,
        lastPatchedItemId: null,
        lastPatchedAspect: null,
        lastPatchedSummary: null,
        lastDeliveryKind: null,
        lastDeliveryScope: null,
        lastDeliveryPacketId: null,
        lastDeliveryBasisId: null,
        currentBasisId: null,
        basisAdvanceCount: 0,
        lastBasisAdvanceFromId: null,
        lastBasisAdvanceToId: null,
        downloadCount: 0,
        readyDownloadCount: 0,
        unavailableDownloadCount: 0,
        incompatibleDownloadCount: 0,
        preservedVisibleValueOnLastRejection: false,
        lastTimeoutOperation: null,
        lastErrorMessage: null,
        visibleValueVersion: 1,
        supersededOperation: null,
      }],
    });
    assert.deepEqual(firstHistory.verificationPackage(), {
      declaration: {
        familyKind: "detail",
        familyId: firstDescriptor.family.familyId,
        canonicalKey: firstDescriptor.canonicalParams.canonicalKey,
        runtimeLineId: firstDescriptor.runtimeLineId,
        scopeId: firstDescriptor.scopeId,
      },
      committedValue: { id: "history" },
      requestPosture: {
        authKind: "anonymous",
        headerNames: [],
        correlationId: null,
        branchId: null,
        basisId: null,
        continuationKind: "none",
        processingKind: "none",
        uploadKind: "none",
      },
      processing: {
        kind: "ready",
        completionKind: "none",
        jobId: null,
        message: null,
      },
      upload: {
        kind: "ready",
        transportKind: "none",
        uploadId: null,
        finalizeRequired: false,
        awaitingProcessing: false,
        message: null,
        hasDescriptor: false,
      },
      lifecycle: {
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
      },
      continuity: {
        continuity: "preserveVisibleValue",
        hasVisibleValue: true,
        visibleValueVersion: 1,
      },
      reconciliation: {
        broadReplace: false,
        narrowItem: false,
        narrowSummary: false,
        aspectNames: [],
        summaryNames: [],
        lastPatchKind: null,
        lastPatchScope: null,
        lastPatchedItemId: null,
        lastPatchedAspect: null,
        lastPatchedSummary: null,
      },
      diagnostics: {
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
            patchedAspect: null,
            patchedSummary: null,
            deliveryKind: null,
            deliveryScope: null,
            deliveryPacketId: null,
            deliveryBasisId: null,
            basisCurrentId: null,
            basisAdvanceFromId: null,
            basisAdvanceToId: null,
            supersededOperation: null,
            timeoutOperation: null,
            errorMessage: null,
            preservedVisibleValueOnLastRejection: false,
          },
        },
      },
      historyReplayRestore: {
        replay: { id: firstSignal.id, family: "replay" },
        lineage: { id: firstSignal.id, family: "lineage" },
        branch: null,
        basis: firstHistory.basis,
        availability: firstHistory.availability,
        lifecycleLength: 1,
        lastLifecycleEvent: "materialized",
      },
      binaryDownload: {
        count: 0,
        readyCount: 0,
        unavailableCount: 0,
        incompatibleCount: 0,
        descriptorKinds: [],
      },
      deliveryProvenance: {
        deliveryCount: 0,
        lastDeliveryKind: null,
        lastDeliveryScope: null,
        lastDeliveryPacketId: null,
        lastDeliveryBasisId: null,
        basisCurrentId: null,
        basisAdvanceCount: 0,
        basisAdvanceFromId: null,
        basisAdvanceToId: null,
      },
      externalCompatibility: {
        kind: "native",
      },
      boundaryPerformanceEnvelope: {
        lifecycleEntryCount: 1,
        downloadDescriptorCount: 0,
        summaryReadShape: "inspectionSummary",
      },
      typedDenials: {
        replay: null,
        replayExact: firstHistory.availability.replayExact,
        lineage: null,
        branch: firstHistory.availability.branch,
        restoreExact: firstHistory.availability.restoreExact,
      },
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
    assert.notEqual(second.signal().id, firstSignal.id);
  } finally {
    await mod.cleanup();
  }
});

test("released resource lines deny operational reads after free and Symbol.dispose", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
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
    await mod.cleanup();
  }
});
