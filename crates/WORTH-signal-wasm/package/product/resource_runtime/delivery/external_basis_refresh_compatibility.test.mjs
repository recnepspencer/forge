import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceRuntime } from "../runtime_fixture/real_resource_signals.mjs";
import {
  createRealCompatibilityDelivery,
  createRealCompatibilityDeliveryLine,
} from "../runtime_fixture/real_delivery_resources.mjs";
import {
  assertDeliveryRequestStateUnchanged,
  captureDeliveryRequestState,
} from "./delivery_request_proof_helpers.mjs";

test("external basis refresh advances basis and reloads through the same line model", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const line = createRealCompatibilityDeliveryLine(
      runtime.resourceMod,
      runtime.signals,
      (_params, request) => ({
        items: [{ id: "demo:1", title: `Basis:${request.context.basisId}` }],
      }),
    );

    const result = line.deliver(
      createRealCompatibilityDelivery(
        runtime.resourceMod,
        runtime.signals,
      ).basisRefresh({
        packetId: "pkt-basis-refresh",
        basisId: "basis-1",
        nextBasisId: "basis-2",
      }),
    );

    assert.deepEqual(result, {
      kind: "basisRefreshed",
      packetId: "pkt-basis-refresh",
      basisId: "basis-1",
      nextBasisId: "basis-2",
      reloadStatus: {
        kind: "fulfilled",
        operation: "delivery",
      },
    });
    assert.deepEqual(line.value(), {
      items: [{ id: "demo:1", title: "Basis:basis-2" }],
    });
    assert.equal(line.request().context.basisId, "basis-2");
    assert.equal(line.diagnostics().lastDeliveryKind, "basisRefresh");
    assert.equal(line.diagnostics().lastDeliveryScope, "basis");
    assert.equal(line.diagnostics().lastOperation, "delivery");
    assert.deepEqual(line.diagnostics().basis, {
      currentBasisId: "basis-2",
      advanceCount: 1,
      lastAdvanceFromBasisId: "basis-1",
      lastAdvanceToBasisId: "basis-2",
    });
    assert.deepEqual(line.diagnosticsSummary().latest, {
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
      deliveryKind: "basisRefresh",
      deliveryScope: "basis",
      deliveryPacketId: "pkt-basis-refresh",
      deliveryBasisId: "basis-1",
      basisCurrentId: "basis-2",
      basisAdvanceFromId: "basis-1",
      basisAdvanceToId: "basis-2",
      effect: line.diagnostics().lastEffect,
      supersededOperation: null,
      timeoutOperation: null,
      errorMessage: null,
      preservedVisibleValueOnLastRejection: false,
    });
    assert.equal(line.history().lifecycle.at(-1)?.event, "delivered");
    assert.equal(line.history().basis.advances.at(-1)?.deliveryKind, "basisRefresh");
    assert.equal(line.history().basis.advances.at(-1)?.deliveryScope, "basis");
  } finally {
    await runtime.cleanup();
  }
});

test("stale external patch denies until basis refresh repairs compatibility, then converges with local refresh truth", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const compatibilityDelivery = createRealCompatibilityDelivery(
      runtime.resourceMod,
      runtime.signals,
    );
    const seenBasisIds = [];
    const line = createRealCompatibilityDeliveryLine(
      runtime.resourceMod,
      runtime.signals,
      (_params, request) => {
        seenBasisIds.push(request.context.basisId);
        return {
          items: [{ id: "demo:1", title: `Basis:${request.context.basisId}` }],
        };
      },
    );

    const beforeRejected = captureDeliveryRequestState(line);
    const rejected = line.deliver(
      compatibilityDelivery.patch({
        packetId: "pkt-stale-external",
        basisId: "basis-2",
        nextBasisId: "basis-3",
        patch: runtime.resourceMod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "Should Reject",
        }),
      }),
    );

    assert.deepEqual(rejected, {
      kind: "basisRejected",
      packetId: "pkt-stale-external",
      expectedBasisId: "basis-1",
      actualBasisId: "basis-2",
    });
    assertDeliveryRequestStateUnchanged(line, beforeRejected);

    const refreshed = line.deliver(
      compatibilityDelivery.basisRefresh({
        packetId: "pkt-refresh-external",
        basisId: "basis-1",
        nextBasisId: "basis-2",
      }),
    );
    const applied = line.deliver(
      compatibilityDelivery.patch({
        packetId: "pkt-current-external",
        basisId: "basis-2",
        nextBasisId: "basis-3",
        patch: runtime.resourceMod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "External Patch After Refresh",
        }),
      }),
    );

    assert.deepEqual(refreshed, {
      kind: "basisRefreshed",
      packetId: "pkt-refresh-external",
      basisId: "basis-1",
      nextBasisId: "basis-2",
      reloadStatus: {
        kind: "fulfilled",
        operation: "delivery",
      },
    });
    assert.deepEqual(applied, {
      kind: "applied",
      deliveryKind: "patch",
      scope: "aspect",
      packetId: "pkt-current-external",
      basisId: "basis-2",
      nextBasisId: "basis-3",
      supersededOperation: null,
    });
    line.refresh();
    assert.deepEqual(line.value(), {
      items: [{ id: "demo:1", title: "Basis:basis-3" }],
    });
    assert.deepEqual(seenBasisIds, ["basis-1", "basis-2", "basis-3"]);
    assert.equal(line.request().context.basisId, "basis-3");
  } finally {
    await runtime.cleanup();
  }
});

test("failed external basis refresh does not advance delivery authority before value truth lands", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const compatibilityDelivery = createRealCompatibilityDelivery(
      runtime.resourceMod,
      runtime.signals,
    );
    let loadCount = 0;
    const line = createRealCompatibilityDeliveryLine(
      runtime.resourceMod,
      runtime.signals,
      (_params, request) => {
        loadCount += 1;
        if (loadCount === 1) {
          return {
            items: [{ id: "demo:1", title: `Basis:${request.context.basisId}` }],
          };
        }
        throw new Error(`basis ${request.context.basisId} reload failed`);
      },
    );

    const beforeRejected = captureDeliveryRequestState(line);
    const refreshed = line.deliver(
      compatibilityDelivery.basisRefresh({
        packetId: "pkt-refresh-failed",
        basisId: "basis-1",
        nextBasisId: "basis-2",
      }),
    );

    assert.deepEqual(refreshed, {
      kind: "basisRefreshed",
      packetId: "pkt-refresh-failed",
      basisId: "basis-1",
      nextBasisId: "basis-2",
      reloadStatus: {
        kind: "rejected",
        operation: "delivery",
        message: "basis basis-2 reload failed",
        continuity: "preservedVisibleValue",
      },
    });
    assert.deepEqual(
      JSON.parse(JSON.stringify(line.request())),
      beforeRejected.request,
    );
    assert.deepEqual(
      JSON.parse(JSON.stringify(line.diagnostics().request)),
      beforeRejected.diagnosticsRequest,
    );
    assert.equal(line.history().lifecycle.at(-1)?.event, "rejected");
    assert.equal(line.history().lifecycle.at(-1)?.currentBasisId, "basis-1");
    assert.equal(line.history().lifecycle.at(-1)?.basisAdvanceCount, 0);
    assert.deepEqual(line.value(), {
      items: [{ id: "demo:1", title: "Basis:basis-1" }],
    });

    const staleNewBasisPatch = line.deliver(
      compatibilityDelivery.patch({
        packetId: "pkt-should-still-reject",
        basisId: "basis-2",
        nextBasisId: "basis-3",
        patch: runtime.resourceMod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "Should Still Reject",
        }),
      }),
    );

    assert.deepEqual(staleNewBasisPatch, {
      kind: "basisRejected",
      packetId: "pkt-should-still-reject",
      expectedBasisId: "basis-1",
      actualBasisId: "basis-2",
    });
  } finally {
    await runtime.cleanup();
  }
});
