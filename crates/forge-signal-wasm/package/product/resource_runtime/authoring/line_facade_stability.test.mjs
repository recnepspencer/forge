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
    assert.deepEqual(firstSignal(), { id: "history" });
    assert.deepEqual(firstHistory, {
      replay: { id: firstSignal.id, family: "replay" },
      lineage: { id: firstSignal.id, family: "lineage" },
      branch: null,
      availability: {
        replay: { kind: "available" },
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
        lastSupersededOperation: null,
        lastInvalidationCause: null,
        lastInvalidationScope: null,
        lastPatchKind: null,
        lastPatchScope: null,
        lastPatchedItemId: null,
        lastPatchedAspect: null,
        lastPatchedSummary: null,
        preservedVisibleValueOnLastRejection: false,
        lastTimeoutOperation: null,
        lastErrorMessage: null,
        visibleValueVersion: 1,
        supersededOperation: null,
      }],
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
