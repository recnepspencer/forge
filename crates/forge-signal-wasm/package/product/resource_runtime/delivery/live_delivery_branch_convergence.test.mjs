import assert from "node:assert/strict";
import test from "node:test";

import { createDeferred } from "../runtime_fixture/async/deferred.mjs";
import {
  createBranchHead,
  createRealResourceRuntime,
} from "../runtime_fixture/real_resource_signals.mjs";
import { createRealDeliveryCollectionLine } from "../runtime_fixture/real_delivery_resources.mjs";
import {
  assertDeliveryRequestStateUnchanged,
  captureDeliveryRequestState,
} from "./delivery_request_proof_helpers.mjs";

test("delivery patch can supersede a pending refresh and become the authoritative local truth", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const deferred = createDeferred();
    let loadCount = 0;
    const seenBasisIds = [];
    const line = createRealDeliveryCollectionLine(
      runtime.mod,
      runtime.signals,
      (_params, request) => {
        seenBasisIds.push(request.context.basisId);
        loadCount += 1;
        if (loadCount === 1) {
          return {
            items: [{ id: "demo:1", title: "First" }],
          };
        }
        return deferred.promise;
      },
    );
    const branch = createBranchHead(runtime.signals, "delivery");
    const snapshotId = Number(runtime.signals.history().branch_snapshot_id(branch.id));

    line.refresh();
    const deliveryResult = line.deliver(
      runtime.mod.resourceDelivery.patch({
        packetId: "pkt-1",
        basisId: "basis-1",
        nextBasisId: "basis-2",
        patch: runtime.mod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "Delivered",
        }),
      }),
    );

    assert.deepEqual(deliveryResult, {
      kind: "applied",
      deliveryKind: "patch",
      scope: "aspect",
      packetId: "pkt-1",
      basisId: "basis-1",
      nextBasisId: "basis-2",
      supersededOperation: "refresh",
    });
    assert.deepEqual(line.value(), {
      items: [{ id: "demo:1", title: "Delivered" }],
    });
    assert.deepEqual(line.status(), {
      kind: "fulfilled",
      operation: "delivery",
    });
    assert.deepEqual(line.freshness(), { kind: "fresh" });
    assert.equal(line.diagnostics().deliveryCount, 1);
    assert.equal(line.diagnostics().supersessionCount, 1);
    assert.equal(line.diagnostics().lastDeliveryKind, "patch");
    assert.equal(line.diagnostics().lastDeliveryPacketId, "pkt-1");
    assert.equal(line.diagnostics().lastPatchScope, "aspect");
    assert.equal(line.request().context.basisId, "basis-2");
    assert.equal(line.diagnostics().request.context.basisId, "basis-2");
    assert.deepEqual(line.diagnostics().basis, {
      currentBasisId: "basis-2",
      advanceCount: 1,
      lastAdvanceFromBasisId: "basis-1",
      lastAdvanceToBasisId: "basis-2",
    });
    assert.deepEqual(line.diagnosticsSummary().counts.basisAdvanceCount, 1);
    assert.deepEqual(line.diagnosticsSummary().latest, {
      invalidationCause: null,
      invalidationScope: null,
      patchKind: "itemAspect",
      patchScope: "aspect",
      patchedItemId: "demo:1",
      patchedAspect: "title",
      patchedSummary: null,
      deliveryKind: "patch",
      deliveryScope: "aspect",
      deliveryPacketId: "pkt-1",
      deliveryBasisId: "basis-1",
      basisCurrentId: "basis-2",
      basisAdvanceFromId: "basis-1",
      basisAdvanceToId: "basis-2",
      supersededOperation: "refresh",
      timeoutOperation: null,
      errorMessage: null,
      preservedVisibleValueOnLastRejection: false,
    });
    assert.deepEqual(
      line.history().lifecycle.slice(-2).map((entry) => ({
        event: entry.event,
        status: entry.status,
        deliveryKind: entry.lastDeliveryKind,
        packetId: entry.lastDeliveryPacketId,
        currentBasisId: entry.currentBasisId,
        basisAdvanceCount: entry.basisAdvanceCount,
        basisAdvanceFromId: entry.lastBasisAdvanceFromId,
        basisAdvanceToId: entry.lastBasisAdvanceToId,
        supersededOperation: entry.supersededOperation,
      })),
      [
        {
          event: "superseded",
          status: {
            kind: "pending",
            operation: "refresh",
            continuity: "preservedVisibleValue",
          },
          deliveryKind: null,
          packetId: null,
          currentBasisId: "basis-1",
          basisAdvanceCount: 0,
          basisAdvanceFromId: null,
          basisAdvanceToId: null,
          supersededOperation: "refresh",
        },
        {
          event: "delivered",
          status: {
            kind: "fulfilled",
            operation: "delivery",
          },
          deliveryKind: "patch",
          packetId: "pkt-1",
          currentBasisId: "basis-2",
          basisAdvanceCount: 1,
          basisAdvanceFromId: "basis-1",
          basisAdvanceToId: "basis-2",
          supersededOperation: null,
        },
      ],
    );

    deferred.resolve({
      items: [{ id: "demo:1", title: "Stale Refresh" }],
    });
    await deferred.promise;
    await Promise.resolve();

    const history = line.history();
    assert.deepEqual(line.value(), {
      items: [{ id: "demo:1", title: "Delivered" }],
    });
    assert.equal(history.branch?.id, branch.id);
    assert.equal(history.branch?.name, branch.name);
    assert.equal(history.branch?.headSnapshotId, snapshotId);
    assert.equal(history.branch?.parentBranchId, 0);
    assert.deepEqual(history.availability, {
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
    assert.ok(Array.isArray(history.replay?.frames));
    assert.ok(history.replay.frames.length > 0);
    assert.deepEqual(seenBasisIds, ["basis-1", "basis-1"]);
  } finally {
    await runtime.cleanup();
  }
});

test("advanced delivery basis is used by later refresh and rejects out-of-order old-basis packets", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const seenBasisIds = [];
    const line = createRealDeliveryCollectionLine(
      runtime.mod,
      runtime.signals,
      (_params, request) => {
        seenBasisIds.push(request.context.basisId);
        return {
          items: [{ id: "demo:1", title: `Basis:${request.context.basisId}` }],
        };
      },
    );

    const delivered = line.deliver(
      runtime.mod.resourceDelivery.replace({
        packetId: "pkt-basis-2",
        basisId: "basis-1",
        nextBasisId: "basis-2",
        nextValue: {
          items: [{ id: "demo:1", title: "Delivered Basis 2" }],
        },
      }),
    );

    assert.deepEqual(delivered, {
      kind: "applied",
      deliveryKind: "replace",
      scope: "line",
      packetId: "pkt-basis-2",
      basisId: "basis-1",
      nextBasisId: "basis-2",
      supersededOperation: null,
    });
    assert.equal(line.request().context.basisId, "basis-2");
    assert.equal(line.diagnostics().request.context.basisId, "basis-2");
    assert.deepEqual(line.diagnostics().basis, {
      currentBasisId: "basis-2",
      advanceCount: 1,
      lastAdvanceFromBasisId: "basis-1",
      lastAdvanceToBasisId: "basis-2",
    });
    const beforeStale = captureDeliveryRequestState(line);

    const stale = line.deliver(
      runtime.mod.resourceDelivery.patch({
        packetId: "pkt-stale-after-advance",
        basisId: "basis-1",
        nextBasisId: "basis-3",
        patch: runtime.mod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "Should Reject",
        }),
      }),
    );

    assert.deepEqual(stale, {
      kind: "basisRejected",
      packetId: "pkt-stale-after-advance",
      expectedBasisId: "basis-2",
      actualBasisId: "basis-1",
    });
    assertDeliveryRequestStateUnchanged(line, beforeStale);

    line.refresh();

    assert.deepEqual(line.value(), {
      items: [{ id: "demo:1", title: "Basis:basis-2" }],
    });
    assert.equal(line.history().lifecycle.at(-1)?.currentBasisId, "basis-2");
    assert.deepEqual(seenBasisIds, ["basis-1", "basis-2"]);
  } finally {
    await runtime.cleanup();
  }
});
