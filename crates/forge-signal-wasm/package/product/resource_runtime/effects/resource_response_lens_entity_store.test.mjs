import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";

test("entity-store responses lower item replacement through entity-store effect loci", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const response = signals.resource.response.entityStore()({
      itemId: (task) => task.id,
      entities: (value) => value.entities,
      replaceEntities: (value, entities) => ({ ...value, entities }),
      aspects: signals.resource.response.objectAspects()({
        title: "title",
      }),
    });
    assert.equal(response.lensProof.topology, "entityStore");
    assert.equal(response.lensProof.capabilityRows.some(
      (row) => row.locus === "entityStore" && row.patchScope === "item",
    ), true);

    const tasks = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/entity-store")
      .response(response)
      .list({
        load: () => ({
          entities: { "task:1": { id: "task:1", title: "First" } },
          total: 1,
        }),
      });
    const line = tasks.line({});
    line.patch(tasks.patch.item({
      itemId: "task:1",
      nextItem: { id: "task:1", title: "Replaced" },
    }));

    const itemEffect = line.diagnostics().lastEffect;
    assert.deepEqual(line.value().entities["task:1"], {
      id: "task:1",
      title: "Replaced",
    });
    assert.deepEqual(itemEffect.locus, {
      kind: "entityStore",
      itemId: "task:1",
    });
    assert.equal(itemEffect.locusProof.lensSource, "resource.response.entityStore<T>()(...)");
    assert.equal(itemEffect.locusProof.topology, "entityStore");
    assert.equal(itemEffect.locusProof.locus, "entityStore");
    assert.equal(itemEffect.locusProof.patchScope, "item");
    assert.equal(itemEffect.locusProof.effectLocusDigest.includes("entityStore"), true);

    line.deliver(signalsMod.resourceDelivery.patch({
      packetId: "pkt-entity-store",
      basisId: null,
      patch: tasks.patch.item({
        itemId: "task:1",
        nextItem: { id: "task:1", title: "Delivered" },
      }),
    }));
    const deliveryEffect = line.diagnostics().lastEffect;
    assert.deepEqual(line.value().entities["task:1"], {
      id: "task:1",
      title: "Delivered",
    });
    assert.equal(deliveryEffect.locus.kind, "entityStore");
    assert.equal(deliveryEffect.locusProof.locus, "entityStore");

    const beforeValue = line.value();
    const beforeEffect = line.diagnostics().lastEffect;
    assert.throws(() => line.patch(tasks.patch.item({
      itemId: "task:1",
      nextItem: { id: "task:2", title: "Wrong identity" },
    })), /preserve item identity/);
    assert.deepEqual(line.value(), beforeValue);
    assert.deepEqual(line.diagnostics().lastEffect, beforeEffect);
  } finally {
    await runtime.cleanup();
  }
});

test("malformed entity-store responses deny before value diagnostics or effect proof changes", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    const response = signals.resource.response.entityStore()({
      itemId: (task) => task.id,
      entities: (value) => value.entities,
      replaceEntities: (value, entities) => ({ ...value, entities }),
    });
    const tasks = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/bad-entity-store")
      .response(response)
      .list({
        load: () => ({ entities: [{ id: "task:1", title: "First" }] }),
      });
    const line = tasks.line({});
    const beforeValue = line.value();
    const beforeEffect = line.diagnostics().lastEffect;

    assert.throws(() => line.patch(tasks.patch.item({
      itemId: "task:1",
      nextItem: { id: "task:1", title: "Replaced" },
    })), /entities\(value\) to return an object record/);
    assert.deepEqual(line.value(), beforeValue);
    assert.equal(line.diagnostics().lastEffect, beforeEffect);
  } finally {
    await runtime.cleanup();
  }
});
