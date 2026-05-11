import assert from "node:assert/strict";
import test from "node:test";

import { createDeferred } from "../runtime_fixture/async/deferred.mjs";
import { createRealLifecycleRuntime } from "../runtime_fixture/real_lifecycle_runtime.mjs";

test("diagnostics summary and lifecycle history stay aligned across patch, invalidation, and rejected refresh", async () => {
  const runtime = await createRealLifecycleRuntime();
  try {
    const { mod, resource } = runtime;
    let shouldFail = false;
    const collection = resource.collection({
      params: mod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        mod.resourceParamIdentity({ workspaceId }, workspaceId),
      itemIdentity: (item) => item.id,
      reconcile: mod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        aspects: mod.resourceItemAspects({
          title: {
            read: (item) => item.title,
            write: (item, title) => ({ ...item, title: String(title) }),
          },
        }),
      }),
      load: ({ workspaceId }) => {
        if (shouldFail) {
          throw new Error(`refresh failed for ${workspaceId}`);
        }
        return {
          items: [{ id: `${workspaceId}:1`, title: "First" }],
        };
      },
    });

    const line = collection.line({ workspaceId: "demo" });
    line.patch(
      mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Patched",
      }),
    );
    line.invalidate();
    shouldFail = true;
    line.refresh();

    const summary = line.diagnosticsSummary();
    const lifecycle = line.history().lifecycle.slice(-3);

    assert.deepEqual(summary.current, {
      status: {
        kind: "rejected",
        operation: "refresh",
        message: "refresh failed for demo",
        continuity: "preservedVisibleValue",
      },
      freshness: { kind: "stale", reason: "refreshRejected" },
      hasVisibleValue: true,
      visibleValueVersion: 2,
    });
    assert.deepEqual(summary.counts, {
      refreshCount: 1,
      revalidateCount: 0,
      retryAttemptCount: 0,
      rejectionCount: 1,
      timeoutCount: 0,
      supersessionCount: 0,
      invalidationCount: 1,
      patchCount: 1,
      deliveryCount: 0,
      basisAdvanceCount: 0,
    });
    assert.deepEqual(summary.latest, {
      invalidationCause: "manualLineInvalidate",
      invalidationScope: "line",
      patchKind: "itemAspect",
      patchScope: "aspect",
      patchedItemId: "demo:1",
      patchedAspect: "title",
      patchedSummary: null,
      deliveryKind: null,
      deliveryScope: null,
      deliveryPacketId: null,
      deliveryBasisId: null,
      basisCurrentId: null,
      basisAdvanceFromId: null,
      basisAdvanceToId: null,
      effect: line.diagnostics().lastEffect,
      supersededOperation: null,
      timeoutOperation: null,
      errorMessage: "refresh failed for demo",
      preservedVisibleValueOnLastRejection: true,
    });
    assert.deepEqual(
      lifecycle.map((entry) => ({
        event: entry.event,
        status: entry.status,
        freshness: entry.freshness,
        patchCount: entry.patchCount,
        invalidationCount: entry.invalidationCount,
        rejectionCount: entry.rejectionCount,
        lastPatchScope: entry.lastPatchScope,
        lastInvalidationCause: entry.lastInvalidationCause,
        lastErrorMessage: entry.lastErrorMessage,
        visibleValueVersion: entry.visibleValueVersion,
      })),
      [
        {
          event: "patched",
          status: { kind: "fulfilled", operation: "initialLoad" },
          freshness: { kind: "fresh" },
          patchCount: 1,
          invalidationCount: 0,
          rejectionCount: 0,
          lastPatchScope: "aspect",
          lastInvalidationCause: null,
          lastErrorMessage: null,
          visibleValueVersion: 2,
        },
        {
          event: "invalidated",
          status: { kind: "fulfilled", operation: "initialLoad" },
          freshness: { kind: "stale", reason: "manualLineInvalidate" },
          patchCount: 1,
          invalidationCount: 1,
          rejectionCount: 0,
          lastPatchScope: "aspect",
          lastInvalidationCause: "manualLineInvalidate",
          lastErrorMessage: null,
          visibleValueVersion: 2,
        },
        {
          event: "rejected",
          status: {
            kind: "rejected",
            operation: "refresh",
            message: "refresh failed for demo",
            continuity: "preservedVisibleValue",
          },
          freshness: { kind: "stale", reason: "refreshRejected" },
          patchCount: 1,
          invalidationCount: 1,
          rejectionCount: 1,
          lastPatchScope: "aspect",
          lastInvalidationCause: "manualLineInvalidate",
          lastErrorMessage: "refresh failed for demo",
          visibleValueVersion: 2,
        },
      ],
    );
  } finally {
    await runtime.cleanup();
  }
});

test("diagnostics summary and lifecycle history stay aligned across upload processing supersession", async () => {
  const runtime = await createRealLifecycleRuntime();
  try {
    const { mod, resource } = runtime;
    const firstDeferred = createDeferred();
    const secondDeferred = createDeferred();
    let loadCount = 0;
    const detail = resource.detail({
      params: mod.resourceParams(),
      processingJob: mod.resourceProcessingJob.poll(),
      uploadTransport: mod.resourceUploadTransport.signed({
        method: "POST",
        finalizeRequired: true,
      }),
      normalizeParams: ({ receiptId }) =>
        mod.resourceParamIdentity({ receiptId }, receiptId),
      load: ({ receiptId }) => {
        loadCount += 1;
        if (loadCount === 1) {
          return mod.resourceUploadResult.uploaded({
            uploadId: `upload:${receiptId}`,
            finalizeRequired: true,
            awaitingProcessing: true,
            message: "processing upload",
          });
        }
        return loadCount === 2 ? firstDeferred.promise : secondDeferred.promise;
      },
    });

    const line = detail.line({ receiptId: "r1" });
    line.refresh();
    line.refresh();

    firstDeferred.resolve({ id: "r1", status: "stale-ready" });
    await firstDeferred.promise;
    await Promise.resolve();

    secondDeferred.resolve({ id: "r1", status: "ready" });
    await secondDeferred.promise;
    await Promise.resolve();

    const summary = line.diagnosticsSummary();
    const lifecycle = line.history().lifecycle.slice(-4);

    assert.deepEqual(summary.current, {
      status: { kind: "fulfilled", operation: "refresh" },
      freshness: { kind: "fresh" },
      hasVisibleValue: true,
      visibleValueVersion: 1,
    });
    assert.equal(summary.counts.refreshCount, 2);
    assert.equal(summary.counts.supersessionCount, 1);
    assert.deepEqual(summary.processing, {
      kind: "ready",
      completionKind: "poll",
      jobId: null,
      message: null,
    });
    assert.deepEqual(summary.upload, {
      kind: "ready",
      transportKind: "signed",
      uploadId: null,
      descriptor: null,
      finalizeRequired: false,
      awaitingProcessing: false,
      message: null,
    });
    assert.deepEqual(summary.latest, {
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
      effect: null,
      supersededOperation: "refresh",
      timeoutOperation: null,
      errorMessage: null,
      preservedVisibleValueOnLastRejection: false,
    });
    assert.deepEqual(
      lifecycle.map((entry) => ({
        event: entry.event,
        status: entry.status,
        supersessionCount: entry.supersessionCount,
        lastSupersededOperation: entry.lastSupersededOperation,
        lastOutcome: entry.lastOutcome,
        visibleValueVersion: entry.visibleValueVersion,
      })),
      [
        {
          event: "pending",
          status: {
            kind: "pending",
            operation: "refresh",
            continuity: "noVisibleValueYet",
          },
          supersessionCount: 0,
          lastSupersededOperation: null,
          lastOutcome: "pending",
          visibleValueVersion: 0,
        },
        {
          event: "superseded",
          status: {
            kind: "pending",
            operation: "refresh",
            continuity: "noVisibleValueYet",
          },
          supersessionCount: 0,
          lastSupersededOperation: null,
          lastOutcome: "pending",
          visibleValueVersion: 0,
        },
        {
          event: "pending",
          status: {
            kind: "pending",
            operation: "refresh",
            continuity: "noVisibleValueYet",
          },
          supersessionCount: 1,
          lastSupersededOperation: "refresh",
          lastOutcome: "pending",
          visibleValueVersion: 0,
        },
        {
          event: "fulfilled",
          status: { kind: "fulfilled", operation: "refresh" },
          supersessionCount: 1,
          lastSupersededOperation: "refresh",
          lastOutcome: "fulfilled",
          visibleValueVersion: 1,
        },
      ],
    );
  } finally {
    await runtime.cleanup();
  }
});
