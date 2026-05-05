import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";
import {
  assertBasisProofUnchanged,
  projectBasisProof,
} from "./delivery_basis_history_proof_helpers.mjs";

function createBasisCloseoutLine(mod, load, signalNamespace) {
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
    load,
  }).line({ workspaceId: "demo" });
}

test("multi-step basis progression stays explicit across delivery, refresh, branch, and replay explainability", async () => {
  const mod = await loadResourceModule();
  try {
    const seenBasisIds = [];
    const signalNamespace = createFakeSignalNamespace("root", {
      current_branch() {
        return {
          id: 45n,
          name: "delivery-closeout",
          parent_branch_id: 12n,
          head_snapshot_id: 96n,
        };
      },
      branch_snapshot() {
        return Object.freeze({ snapshotRestoreToken: "branch-45-snapshot" });
      },
      restore_exact_branch_snapshot() {},
    });
    const line = createBasisCloseoutLine(
      mod,
      (_params, request) => {
        seenBasisIds.push(request.context.basisId);
        return {
          items: [{ id: "demo:1", title: `Load:${request.context.basisId}` }],
        };
      },
      signalNamespace,
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
    line.refresh();
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
    line.refresh();

    const proof = projectBasisProof(line);

    assert.deepEqual(line.value(), {
      items: [{ id: "demo:1", title: "Load:basis-3" }],
    });
    assert.deepEqual(proof.diagnosticsBasis, {
      currentBasisId: "basis-3",
      advanceCount: 2,
      lastAdvanceFromBasisId: "basis-2",
      lastAdvanceToBasisId: "basis-3",
    });
    assert.deepEqual(proof.summaryBasis, {
      count: 2,
      currentBasisId: "basis-3",
      fromBasisId: "basis-2",
      toBasisId: "basis-3",
    });
    assert.deepEqual(proof.historyBasis, {
      currentBasisId: "basis-3",
      advanceCount: 2,
      lastAdvanceFromId: "basis-2",
      lastAdvanceToId: "basis-3",
      advances: [
        {
          sequence: 2,
          event: "delivered",
          operation: "delivery",
          deliveryKind: "replace",
          deliveryScope: "line",
          deliveryPacketId: "pkt-basis-2",
          deliveryBasisId: "basis-1",
          fromBasisId: "basis-1",
          toBasisId: "basis-2",
          currentBasisId: "basis-2",
        },
        {
          sequence: 4,
          event: "delivered",
          operation: "delivery",
          deliveryKind: "patch",
          deliveryScope: "aspect",
          deliveryPacketId: "pkt-basis-3",
          deliveryBasisId: "basis-2",
          fromBasisId: "basis-2",
          toBasisId: "basis-3",
          currentBasisId: "basis-3",
        },
      ],
    });
    assert.deepEqual(proof.branch, {
      id: 45,
      name: "delivery-closeout",
      parentBranchId: 12,
      headSnapshotId: 96,
    });
    assert.deepEqual(proof.availability, {
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
        branchId: 45,
        snapshotId: 96,
      },
    });
    assert.deepEqual(seenBasisIds, ["basis-1", "basis-2", "basis-3"]);
    assert.equal(proof.replay.id, line.signal().id);
  } finally {
    await mod.cleanup();
  }
});

test("stale or duplicate packets after multi-step basis progression cannot rewrite the basis proof surface", async () => {
  const mod = await loadResourceModule();
  try {
    const line = createBasisCloseoutLine(
      mod,
      (_params, request) => ({
        items: [{ id: "demo:1", title: `Load:${request.context.basisId}` }],
      }),
      createFakeSignalNamespace("root", {
        current_branch() {
          return {
            id: 51n,
            name: "delivery-proof",
            parent_branch_id: null,
            head_snapshot_id: 105n,
          };
        },
        branch_snapshot() {
          return Object.freeze({ snapshotRestoreToken: "branch-51-snapshot" });
        },
        restore_exact_branch_snapshot() {},
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
    const before = projectBasisProof(line);

    const duplicate = line.deliver(
      mod.resourceDelivery.patch({
        packetId: "pkt-basis-3",
        basisId: "basis-3",
        nextBasisId: "basis-4",
        patch: mod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "ignored duplicate",
        }),
      }),
    );
    const stale = line.deliver(
      mod.resourceDelivery.patch({
        packetId: "pkt-stale",
        basisId: "basis-2",
        nextBasisId: "basis-4",
        patch: mod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "ignored stale",
        }),
      }),
    );

    assert.deepEqual(duplicate, {
      kind: "duplicateIgnored",
      packetId: "pkt-basis-3",
      deliveryKind: "patch",
    });
    assert.deepEqual(stale, {
      kind: "basisRejected",
      packetId: "pkt-stale",
      expectedBasisId: "basis-3",
      actualBasisId: "basis-2",
    });
    assertBasisProofUnchanged(line, before);
  } finally {
    await mod.cleanup();
  }
});

test("restore after mixed delivery and refresh reconstructs local line truth without erasing basis history", async () => {
  const mod = await loadResourceModule();
  try {
    let restoreMode = false;
    const line = createBasisCloseoutLine(
      mod,
      (_params, request) => ({
        items: [{
          id: "demo:1",
          title: restoreMode
            ? "Restored Snapshot"
            : `Load:${request.context.basisId}`,
        }],
      }),
      createFakeSignalNamespace("root", {
        current_branch() {
          return {
            id: 63n,
            name: "delivery-restore",
            parent_branch_id: 7n,
            head_snapshot_id: 126n,
          };
        },
        restore_branch_snapshot_by_id() {
          restoreMode = true;
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
    line.refresh();
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

    const beforeBasis = projectBasisProof(line).historyBasis;
    const restored = line.history().restoreExact();
    const after = projectBasisProof(line);

    assert.deepEqual(restored, {
      kind: "restored",
      mode: "SameRuntimeBranchExact",
      branchId: 63,
      snapshotId: 126,
      basisCurrentId: "basis-3",
      basisAdvanceCount: 2,
      reloadStatus: {
        kind: "fulfilled",
        operation: "restore",
      },
    });
    assert.deepEqual(line.value(), {
      items: [{ id: "demo:1", title: "Restored Snapshot" }],
    });
    assert.deepEqual(line.status(), {
      kind: "fulfilled",
      operation: "restore",
    });
    assert.equal(line.diagnostics().lastOperation, "restore");
    assert.equal(line.history().lifecycle.at(-1)?.event, "restored");
    assert.deepEqual(after.historyBasis, beforeBasis);
    assert.equal(after.requestBasisId, "basis-3");
  } finally {
    await mod.cleanup();
  }
});
