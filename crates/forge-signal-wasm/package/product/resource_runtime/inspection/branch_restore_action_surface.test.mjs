import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";
import { projectBasisProof } from "../delivery/delivery_basis_history_proof_helpers.mjs";

function createRestoreCollectionLine(mod, signalNamespace) {
  return mod.createResourceNamespace(signalNamespace, {}).collection({
    params: mod.resourceParams(),
    normalizeParams: ({ workspaceId }) =>
      mod.resourceParamIdentity({ workspaceId }, workspaceId),
    requestContext: mod.resourceRequestContext({ basisId: "basis-1" }),
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
    load: (_params, request) => ({
      items: [{ id: "demo:1", title: `Load:${request.context.basisId}` }],
    }),
  }).line({ workspaceId: "demo" });
}

test("line history restoreExact targets the current branch head and preserves basis evidence", async () => {
  const mod = await loadResourceModule();
  try {
    const calls = [];
    let restored = false;
    const line = createRestoreCollectionLine(
      mod,
      createFakeSignalNamespace("root", {
        current_branch() {
          return {
            id: 72n,
            name: "restore-branch",
            parent_branch_id: 9n,
            head_snapshot_id: 144n,
          };
        },
        restore_branch_snapshot_by_id(branchId, snapshotId) {
          restored = true;
          calls.push([branchId, snapshotId]);
        },
      }),
    );

    line.deliver(
      mod.resourceDelivery.replace({
        packetId: "pkt-basis-2",
        basisId: "basis-1",
        nextBasisId: "basis-2",
        nextValue: {
          items: [{ id: "demo:1", title: "Delivered Basis 2" }],
        },
      }),
    );
    line.deliver(
      mod.resourceDelivery.patch({
        packetId: "pkt-basis-3",
        basisId: "basis-2",
        nextBasisId: "basis-3",
        patch: mod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "Delivered Basis 3",
        }),
      }),
    );

    const result = line.history().restoreExact();

    assert.deepEqual(result, {
      kind: "restored",
      mode: "SameRuntimeBranchExact",
      branchId: 72,
      snapshotId: 144,
      basisCurrentId: "basis-3",
      basisAdvanceCount: 2,
      reloadStatus: {
        kind: "fulfilled",
        operation: "restore",
      },
    });
    assert.deepEqual(calls, [[72n, 144n]]);
    assert.equal(restored, true);
    assert.deepEqual(line.value(), {
      items: [{ id: "demo:1", title: "Load:basis-3" }],
    });
    assert.deepEqual(line.status(), {
      kind: "fulfilled",
      operation: "restore",
    });
    assert.equal(line.history().lifecycle.at(-1)?.event, "restored");
    assert.deepEqual(line.history().availability.restoreExact, {
      kind: "available",
      mode: "SameRuntimeBranchExact",
      branchId: 72,
      snapshotId: 144,
    });
  } finally {
    await mod.cleanup();
  }
});

test("line history restoreExact treats by-id runtime support as exact-restore capable", async () => {
  const mod = await loadResourceModule();
  try {
    let invoked = false;
    const line = mod.createResourceNamespace(
      createFakeSignalNamespace("root", {
        current_branch() {
          return {
            id: 8n,
            name: "by-id-only",
            parent_branch_id: null,
            head_snapshot_id: 16n,
          };
        },
        restore_branch_snapshot_by_id() {
          invoked = true;
        },
      }),
      {},
    ).detail({
      params: mod.resourceParams(),
      normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
      load: ({ id }) => ({ id }),
    }).line({ id: "plain" });

    assert.deepEqual(line.history().availability.restoreExact, {
      kind: "available",
      mode: "SameRuntimeBranchExact",
      branchId: 8,
      snapshotId: 16,
    });
    assert.equal(line.history().restoreExact().kind, "restored");
    assert.equal(invoked, true);
  } finally {
    await mod.cleanup();
  }
});

test("line history restoreExact can use exact restore with a branch snapshot artifact when by-id restore is absent", async () => {
  const mod = await loadResourceModule();
  try {
    const calls = [];
    const snapshot = Object.freeze({
      snapshotRestoreToken: "branch-18-snapshot",
    });
    const line = mod.createResourceNamespace(
      createFakeSignalNamespace("root", {
        current_branch() {
          return {
            id: 18n,
            name: "exact-only",
            parent_branch_id: 2n,
            head_snapshot_id: 36n,
          };
        },
        branch_snapshot(branchId) {
          calls.push(["snapshot", branchId]);
          return snapshot;
        },
        restore_exact_branch_snapshot(branchId, snapshotValue) {
          calls.push(["restore", branchId, snapshotValue]);
        },
      }),
      {},
    ).detail({
      params: mod.resourceParams(),
      normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
      load: ({ id }) => ({ id }),
    }).line({ id: "plain" });

    const result = line.history().restoreExact();

    assert.deepEqual(result, {
      kind: "restored",
      mode: "SameRuntimeBranchExact",
      branchId: 18,
      snapshotId: 36,
      basisCurrentId: null,
      basisAdvanceCount: 0,
      reloadStatus: {
        kind: "fulfilled",
        operation: "restore",
      },
    });
    assert.deepEqual(calls, [
      ["snapshot", 18n],
      ["restore", 18n, snapshot],
    ]);
  } finally {
    await mod.cleanup();
  }
});

test("line history restoreExact returns explicit unavailable or runtimeRejected artifacts without rewriting basis proof", async () => {
  const mod = await loadResourceModule();
  try {
    const missingHead = createRestoreCollectionLine(
      mod,
      createFakeSignalNamespace("root", {
        current_branch() {
          return {
            id: 15n,
            name: "missing-head",
            parent_branch_id: null,
            head_snapshot_id: null,
          };
        },
      }),
    );
    missingHead.deliver(
      mod.resourceDelivery.replace({
        packetId: "pkt-basis-2",
        basisId: "basis-1",
        nextBasisId: "basis-2",
        nextValue: {
          items: [{ id: "demo:1", title: "Delivered Basis 2" }],
        },
      }),
    );
    const missingHeadBefore = projectBasisProof(missingHead);
    const missingHeadResult = missingHead.history().restoreExact();

    assert.deepEqual(missingHeadResult, {
      kind: "unavailable",
      reason: "branchHeadUnavailable",
      detail:
        "resource line exact branch restore is unavailable because branch 15 has no head snapshot",
      basisCurrentId: "basis-2",
      basisAdvanceCount: 1,
    });
    assert.deepEqual(projectBasisProof(missingHead), missingHeadBefore);

    const missingSnapshotArtifact = mod.createResourceNamespace(
      createFakeSignalNamespace("root", {
        current_branch() {
          return {
            id: 19n,
            name: "missing-snapshot-artifact",
            parent_branch_id: null,
            head_snapshot_id: 38n,
          };
        },
        restore_exact_branch_snapshot() {},
      }),
      {},
    ).detail({
      params: mod.resourceParams(),
      normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
      load: ({ id }) => ({ id }),
    }).line({ id: "plain" });

    assert.deepEqual(missingSnapshotArtifact.history().availability.restoreExact, {
      kind: "unavailable",
      reason: "unsupportedByRuntime",
      detail:
        "resource line exact branch restore is unavailable because the Signals runtime does not expose restore_branch_snapshot_by_id(...) or a restore_exact_branch_snapshot(...) + branch_snapshot(...) pair",
    });
    assert.deepEqual(missingSnapshotArtifact.history().restoreExact(), {
      kind: "unavailable",
      reason: "unsupportedByRuntime",
      detail:
        "resource line exact branch restore is unavailable because the Signals runtime does not expose restore_branch_snapshot_by_id(...) or a restore_exact_branch_snapshot(...) + branch_snapshot(...) pair",
      basisCurrentId: null,
      basisAdvanceCount: 0,
    });

    const rejected = createRestoreCollectionLine(
      mod,
      createFakeSignalNamespace("root", {
        current_branch() {
          return {
            id: 22n,
            name: "rejecting-restore",
            parent_branch_id: 1n,
            head_snapshot_id: 44n,
          };
        },
        restore_branch_snapshot_by_id() {
          throw new Error("snapshot 44 is no longer retained");
        },
      }),
    );
    rejected.deliver(
      mod.resourceDelivery.replace({
        packetId: "pkt-basis-2",
        basisId: "basis-1",
        nextBasisId: "basis-2",
        nextValue: {
          items: [{ id: "demo:1", title: "Delivered Basis 2" }],
        },
      }),
    );
    rejected.deliver(
      mod.resourceDelivery.patch({
        packetId: "pkt-basis-3",
        basisId: "basis-2",
        nextBasisId: "basis-3",
        patch: mod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "Delivered Basis 3",
        }),
      }),
    );
    const rejectedBefore = projectBasisProof(rejected);
    const rejectedResult = rejected.history().restoreExact();

    assert.deepEqual(rejectedResult, {
      kind: "unavailable",
      reason: "runtimeRejected",
      detail:
        "resource line exact branch restore is unavailable because restore execution failed: snapshot 44 is no longer retained",
      basisCurrentId: "basis-3",
      basisAdvanceCount: 2,
    });
    assert.deepEqual(projectBasisProof(rejected), rejectedBefore);
  } finally {
    await mod.cleanup();
  }
});
