import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

test("resource line diagnostics summary groups current state and unavailable explainability clearly", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
      load: ({ id }) => ({ id }),
    });

    const summary = detail.line({ id: "plain" }).diagnosticsSummary();

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
    await mod.cleanup();
  }
});

test("resource line diagnostics summary does not materialize replay or lineage artifacts", async () => {
  const mod = await loadResourceModule();
  try {
    let replayReads = 0;
    let lineageReads = 0;
    const signalNamespace = createFakeSignalNamespace("root", {
      replay_for() {
        replayReads += 1;
        throw new Error("diagnosticsSummary should not call replay_for(...)");
      },
      lineage_for() {
        lineageReads += 1;
        throw new Error("diagnosticsSummary should not call lineage_for(...)");
      },
      current_branch() {
        return {
          id: 2n,
          name: "summary-only",
          parent_branch_id: null,
          head_snapshot_id: null,
        };
      },
    });
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
      load: ({ id }) => ({ id }),
    });

    const summary = detail.line({ id: "plain" }).diagnosticsSummary();

    assert.equal(replayReads, 0);
    assert.equal(lineageReads, 0);
    assert.deepEqual(summary.explainability.restoreExact, {
      kind: "unavailable",
      reason: "branchHeadUnavailable",
      detail:
        "resource line exact branch restore is unavailable because branch 2 has no head snapshot",
    });
  } finally {
    await mod.cleanup();
  }
});

test("resource line diagnostics summary keeps latest patch truth aligned with explainability availability", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace("root", {
      current_branch() {
        return {
          id: 11n,
          name: "collections",
          parent_branch_id: 1n,
          head_snapshot_id: 21n,
        };
      },
      branch_snapshot() {
        return Object.freeze({ snapshotRestoreToken: "branch-11-snapshot" });
      },
      restore_exact_branch_snapshot() {},
    });
    const resource = mod.createResourceNamespace(signalNamespace, {});
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
      mod.resourcePatch.itemAspect({
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
    assert.deepEqual(summary.explainability, {
      replay: { kind: "available" },
      replayExact: {
        kind: "unavailable",
        reason: "unsupportedByRuntime",
        detail:
          "resource line exact replay is unavailable because the Signals runtime does not expose replay_signal_by_id(...)",
      },
      lineage: { kind: "available" },
      branch: { kind: "available" },
      restoreExact: {
        kind: "available",
        mode: "SameRuntimeBranchExact",
        branchId: 11,
        snapshotId: 21,
      },
    });
  } finally {
    await mod.cleanup();
  }
});
