import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceRuntime } from "../runtime_fixture/real_resource_signals.mjs";
import { createRealDeliveryCollectionLine } from "../runtime_fixture/real_delivery_resources.mjs";
import {
  assertLineStateUnchanged,
  captureLineState,
} from "../reconciliation/reconciliation_proof_helpers.mjs";

test("duplicate delivery packets are ignored without side effects", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const line = createRealDeliveryCollectionLine(
      runtime.mod,
      runtime.signals,
      () => ({
        items: [{ id: "demo:1", title: "First" }],
      }),
    );

    const first = line.deliver(
      runtime.mod.resourceDelivery.patch({
        packetId: "pkt-dup",
        patch: runtime.mod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "Delivered",
        }),
      }),
    );
    const beforeDuplicate = captureLineState(line);
    const duplicate = line.deliver(
      runtime.mod.resourceDelivery.patch({
        packetId: "pkt-dup",
        patch: runtime.mod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "Ignored",
        }),
      }),
    );

    assert.equal(first.kind, "applied");
    assert.deepEqual(duplicate, {
      kind: "duplicateIgnored",
      packetId: "pkt-dup",
      deliveryKind: "patch",
    });
    assertLineStateUnchanged(line, beforeDuplicate);
  } finally {
    await runtime.cleanup();
  }
});

test("stale basis delivery is rejected explicitly without side effects", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const line = createRealDeliveryCollectionLine(
      runtime.mod,
      runtime.signals,
      () => ({
        items: [{ id: "demo:1", title: "First" }],
      }),
    );
    const before = captureLineState(line);

    const rejected = line.deliver(
      runtime.mod.resourceDelivery.replace({
        packetId: "pkt-stale",
        basisId: "basis-2",
        nextValue: {
          items: [{ id: "demo:1", title: "Wrong Basis" }],
        },
      }),
    );

    assert.deepEqual(rejected, {
      kind: "basisRejected",
      packetId: "pkt-stale",
      expectedBasisId: "basis-1",
      actualBasisId: "basis-2",
    });
    assertLineStateUnchanged(line, before);
  } finally {
    await runtime.cleanup();
  }
});

test("delivery invalidation is recorded as delivery provenance instead of manual invalidation folklore", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const line = createRealDeliveryCollectionLine(
      runtime.mod,
      runtime.signals,
      () => ({
        items: [{ id: "demo:1", title: "First" }],
      }),
    );

    const result = line.deliver(
      runtime.mod.resourceDelivery.invalidate({
        packetId: "pkt-invalidate",
        basisId: "basis-1",
      }),
    );

    assert.deepEqual(result, {
      kind: "applied",
      deliveryKind: "invalidate",
      scope: "invalidate",
      packetId: "pkt-invalidate",
      basisId: "basis-1",
      nextBasisId: "basis-1",
      supersededOperation: null,
    });
    assert.deepEqual(line.status(), {
      kind: "fulfilled",
      operation: "delivery",
    });
    assert.deepEqual(line.freshness(), {
      kind: "stale",
      reason: "deliveryInvalidate",
    });
    assert.equal(line.diagnostics().invalidationCount, 1);
    assert.equal(line.diagnostics().deliveryCount, 1);
    assert.equal(line.diagnostics().lastInvalidationCause, "deliveryInvalidate");
    assert.equal(line.diagnostics().lastDeliveryKind, "invalidate");
    assert.deepEqual(line.diagnostics().basis, {
      currentBasisId: "basis-1",
      advanceCount: 0,
      lastAdvanceFromBasisId: null,
      lastAdvanceToBasisId: null,
    });
    assert.deepEqual(line.diagnosticsSummary().latest, {
      invalidationCause: "deliveryInvalidate",
      invalidationScope: "line",
      patchKind: null,
      patchScope: null,
      patchedItemId: null,
      patchedAspect: null,
      patchedSummary: null,
      deliveryKind: "invalidate",
      deliveryScope: "invalidate",
      deliveryPacketId: "pkt-invalidate",
      deliveryBasisId: "basis-1",
      basisCurrentId: "basis-1",
      basisAdvanceFromId: null,
      basisAdvanceToId: null,
      effect: line.diagnostics().lastEffect,
      supersededOperation: null,
      timeoutOperation: null,
      errorMessage: null,
      preservedVisibleValueOnLastRejection: false,
    });
    assert.equal(line.history().lifecycle.at(-1)?.event, "delivered");
    assert.equal(line.history().lifecycle.at(-1)?.basisAdvanceCount, 0);
  } finally {
    await runtime.cleanup();
  }
});

test("delivery patch still obeys declared reconciliation legality and denies undeclared narrow paths without side effects", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const line = createRealDeliveryCollectionLine(
      runtime.mod,
      runtime.signals,
      () => ({
        items: [{ id: "demo:1", title: "First" }],
      }),
    );
    const before = captureLineState(line);

    assert.throws(
      () =>
        line.deliver(
          runtime.mod.resourceDelivery.patch({
            packetId: "pkt-illegal",
            basisId: "basis-1",
            patch: runtime.mod.resourcePatch.itemAspect({
              itemId: "demo:1",
              aspect: "status",
              value: "illegal",
            }),
          }),
        ),
      /do not admit itemAspect patch\(\.\.\.\) for undeclared aspect "status"/,
    );

    assertLineStateUnchanged(line, before);
  } finally {
    await runtime.cleanup();
  }
});
