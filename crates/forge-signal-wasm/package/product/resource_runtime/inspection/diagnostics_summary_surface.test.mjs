import assert from "node:assert/strict";
import test from "node:test";

import {
  createBranchHead,
  createRealResourceNamespace,
  createRealResourceRuntime,
} from "../runtime_fixture/real_resource_signals.mjs";

function createDetailLine(resourceMod, signals, historyOverrides = null, id = "detail") {
  return createRealResourceNamespace(resourceMod, signals, historyOverrides)
    .detail({
      params: resourceMod.resourceParams(),
      normalizeParams: ({ id }) => resourceMod.resourceParamIdentity({ id }, id),
      load: ({ id }) => ({ id }),
    })
    .line({ id });
}

test("resource line diagnostics summary groups current state and unavailable explainability clearly", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const summary = createDetailLine(
      runtime.resourceMod,
      runtime.signals,
      {
        current_branch: undefined,
      },
      "plain",
    ).diagnosticsSummary();

    assert.deepEqual(summary.current, {
      status: { kind: "fulfilled", operation: "initialLoad" },
      freshness: { kind: "fresh" },
      hasVisibleValue: true,
      visibleValueVersion: 1,
    });
    assert.deepEqual(summary.activity, {
      lastOperation: "initialLoad",
      lastOutcome: "fulfilled",
      pendingOperation: null,
      continuity: "preserveVisibleValue",
      freshnessPolicy: "stable",
    });
    assert.deepEqual(summary.counts, {
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
    });
    assert.deepEqual(summary.download, {
      count: 0,
      readyCount: 0,
      unavailableCount: 0,
      incompatibleCount: 0,
      descriptors: [],
    });
    assert.deepEqual(summary.explainability, {
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
  } finally {
    await runtime.cleanup();
  }
});

test("resource line diagnostics summary does not materialize replay or lineage artifacts", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    let replayReads = 0;
    let lineageReads = 0;
    const summary = createDetailLine(
      runtime.resourceMod,
      runtime.signals,
      {
        replay_for() {
          replayReads += 1;
          throw new Error("diagnosticsSummary should not call replay_for(...)");
        },
        lineage_for() {
          lineageReads += 1;
          throw new Error("diagnosticsSummary should not call lineage_for(...)");
        },
        current_branch: undefined,
      },
      "plain",
    ).diagnosticsSummary();

    assert.equal(replayReads, 0);
    assert.equal(lineageReads, 0);
    assert.deepEqual(summary.explainability.branch, {
      kind: "unavailable",
      reason: "unsupportedByRuntime",
      detail:
        "resource line branch history is unavailable because the Signals runtime does not expose current_branch(...)",
    });
  } finally {
    await runtime.cleanup();
  }
});

test("resource line diagnostics summary keeps latest patch truth aligned with real explainability availability", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const branch = createBranchHead(runtime.signals, "collections");
    const resource = createRealResourceNamespace(
      runtime.resourceMod,
      runtime.signals,
    );
    const collection = resource.collection({
      params: runtime.resourceMod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        runtime.resourceMod.resourceParamIdentity({ workspaceId }, workspaceId),
      itemIdentity: (item) => item.id,
      reconcile: runtime.resourceMod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        aspects: runtime.resourceMod.resourceItemAspects({
          title: {
            read: (item) => item.title,
            write: (item, title) => ({ ...item, title }),
          },
        }),
      }),
      load: ({ workspaceId }) => ({
        items: [{ id: `${workspaceId}:1`, title: "First" }],
      }),
    });

    const line = collection.line({ workspaceId: "demo" });
    line.patch(
      runtime.resourceMod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Updated",
      }),
    );
    const summary = line.diagnosticsSummary();

    assert.deepEqual(summary.latest, {
      invalidationCause: null,
      invalidationScope: null,
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
      supersededOperation: null,
      timeoutOperation: null,
      errorMessage: null,
      preservedVisibleValueOnLastRejection: false,
    });
    assert.deepEqual(summary.download, {
      count: 0,
      readyCount: 0,
      unavailableCount: 0,
      incompatibleCount: 0,
      descriptors: [],
    });
    assert.deepEqual(summary.explainability.replayExact, {
      kind: "unavailable",
      reason: "unsupportedByRuntime",
      detail:
        "resource line exact replay is unavailable because the Signals runtime does not expose replay_signal_by_id(...)",
    });
    assert.deepEqual(summary.explainability.lineage, { kind: "available" });
    assert.deepEqual(summary.explainability.branch, { kind: "available" });
    assert.equal(summary.explainability.restoreExact.kind, "available");
    assert.equal(summary.explainability.restoreExact.mode, "SameRuntimeBranchExact");
    assert.equal(summary.explainability.restoreExact.branchId, branch.id);
    assert.equal(
      Number.isInteger(summary.explainability.restoreExact.snapshotId),
      true,
    );
  } finally {
    await runtime.cleanup();
  }
});
