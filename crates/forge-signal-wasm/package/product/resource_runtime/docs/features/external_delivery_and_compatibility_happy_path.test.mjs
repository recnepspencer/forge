import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceRuntime } from "../../runtime_fixture/real_resource_signals.mjs";
import {
  createRealCompatibilityDelivery,
  createRealCompatibilityDeliveryLine,
} from "../../runtime_fixture/real_delivery_resources.mjs";

test("external delivery and compatibility doc happy path covers basis refresh on an external collection line", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const line = createRealCompatibilityDeliveryLine(
      runtime.resourceMod,
      runtime.signals,
      (_params, request) => ({
        items: [{ id: "t1", title: `Basis:${request.context.basisId}` }],
      }),
    );

    const result = line.deliver(
      createRealCompatibilityDelivery(
        runtime.resourceMod,
        runtime.signals,
      ).basisRefresh({
        packetId: "pkt-refresh",
        basisId: "basis-1",
        nextBasisId: "basis-2",
      }),
    );

    assert.equal(result.kind, "basisRefreshed");
    assert.equal(line.request().context.basisId, "basis-2");
    assert.deepEqual(line.value(), {
      items: [{ id: "t1", title: "Basis:basis-2" }],
    });
    assert.equal(line.diagnostics().lastDeliveryKind, "basisRefresh");
  } finally {
    await runtime.cleanup();
  }
});
