import assert from "node:assert/strict";
import test from "node:test";

import {
  createBranchHead,
  createRealResourceRuntime,
} from "../runtime_fixture/real_resource_signals.mjs";
import { createRealDeliveryCollectionLine } from "../runtime_fixture/real_delivery_resources.mjs";
import {
  assertBasisProofUnchanged,
  projectBasisProof,
} from "./delivery_basis_history_proof_helpers.mjs";

test("multi-step basis progression stays explicit across delivery, refresh, branch, and replay explainability", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const seenBasisIds = [];
    const line = createRealDeliveryCollectionLine(
      runtime.mod,
      runtime.signals,
      (_params, request) => {
        seenBasisIds.push(request.context.basisId);
        return {
          items: [{ id: "demo:1", title: `Load:${request.context.basisId}` }],
        };
      },
    );
    const branch = createBranchHead(runtime.signals, "delivery-closeout");
    const snapshotId = Number(runtime.signals.history().branch_snapshot_id(branch.id));

    line.deliver(
      runtime.mod.resourceDelivery.replace({
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
      runtime.mod.resourceDelivery.patch({
        packetId: "pkt-basis-3",
        basisId: "basis-2",
        nextBasisId: "basis-3",
        patch: runtime.mod.resourcePatch.itemAspect({
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
    assert.equal(proof.branch?.id, branch.id);
    assert.equal(proof.branch?.name, branch.name);
    assert.equal(proof.branch?.parentBranchId, 0);
    assert.equal(proof.branch?.headSnapshotId, snapshotId);
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
        branchId: branch.id,
        snapshotId,
      },
    });
    assert.deepEqual(seenBasisIds, ["basis-1", "basis-2", "basis-3"]);
    assert.ok(Array.isArray(proof.replay?.frames));
    assert.ok(proof.replay.frames.length > 0);
  } finally {
    await runtime.cleanup();
  }
});

test("stale or duplicate packets after multi-step basis progression cannot rewrite the basis proof surface", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const line = createRealDeliveryCollectionLine(
      runtime.mod,
      runtime.signals,
      (_params, request) => ({
        items: [{ id: "demo:1", title: `Load:${request.context.basisId}` }],
      }),
    );
    createBranchHead(runtime.signals, "delivery-proof");

    line.deliver(
      runtime.mod.resourceDelivery.replace({
        packetId: "pkt-basis-2",
        basisId: "basis-1",
        nextBasisId: "basis-2",
        nextValue: {
          items: [{ id: "demo:1", title: "Delivered Basis 2" }],
        },
      }),
    );
    line.deliver(
      runtime.mod.resourceDelivery.patch({
        packetId: "pkt-basis-3",
        basisId: "basis-2",
        nextBasisId: "basis-3",
        patch: runtime.mod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "Delivered Basis 3",
        }),
      }),
    );
    const before = projectBasisProof(line);

    const duplicate = line.deliver(
      runtime.mod.resourceDelivery.patch({
        packetId: "pkt-basis-3",
        basisId: "basis-3",
        nextBasisId: "basis-4",
        patch: runtime.mod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "ignored duplicate",
        }),
      }),
    );
    const stale = line.deliver(
      runtime.mod.resourceDelivery.patch({
        packetId: "pkt-stale",
        basisId: "basis-2",
        nextBasisId: "basis-4",
        patch: runtime.mod.resourcePatch.itemAspect({
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
    await runtime.cleanup();
  }
});

test("restore after mixed delivery and refresh reconstructs local line truth without erasing basis history", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const line = createRealDeliveryCollectionLine(
      runtime.mod,
      runtime.signals,
      (_params, request) => ({
        items: [{ id: "demo:1", title: `Load:${request.context.basisId}` }],
      }),
    );
    const branch = createBranchHead(runtime.signals, "delivery-restore");
    const snapshotId = Number(runtime.signals.history().branch_snapshot_id(branch.id));

    line.deliver(
      runtime.mod.resourceDelivery.replace({
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
      runtime.mod.resourceDelivery.patch({
        packetId: "pkt-basis-3",
        basisId: "basis-2",
        nextBasisId: "basis-3",
        patch: runtime.mod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "Delivered Basis 3",
        }),
      }),
    );

    const before = projectBasisProof(line);
    const restored = line.history().restoreExact();
    const after = projectBasisProof(line);

    assert.deepEqual(restored, {
      kind: "restored",
      mode: "SameRuntimeBranchExact",
      branchId: branch.id,
      snapshotId,
      basisCurrentId: "basis-3",
      basisAdvanceCount: 2,
      reloadStatus: {
        kind: "fulfilled",
        operation: "restore",
      },
    });
    assert.deepEqual(line.value(), {
      items: [{ id: "demo:1", title: "Load:basis-3" }],
    });
    assert.deepEqual(line.status(), {
      kind: "fulfilled",
      operation: "restore",
    });
    assert.equal(line.diagnostics().lastOperation, "restore");
    assert.equal(line.history().lifecycle.at(-1)?.event, "restored");
    assert.equal(after.requestBasisId, "basis-3");
    assert.deepEqual(after.historyBasis, before.historyBasis);
  } finally {
    await runtime.cleanup();
  }
});
