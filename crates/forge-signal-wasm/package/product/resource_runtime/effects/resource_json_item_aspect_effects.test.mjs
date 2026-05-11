import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";
import { createBranchHead } from "../runtime_fixture/real_resource_signals.mjs";

test("JSON path item aspects patch local delivery and rollback through jsonItemAspect loci", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    createBranchHead(signals, "json-path-aspect");
    const response = createJsonPathResponse(signals);
    const tasks = signals.api({
      effects: signals.resource.effects.branchNative(),
    }).url("/json-path-tasks")
      .response(response)
      .list({
        load: () => ({
          tasks: [{
            id: "t1",
            metadata: { priority: 1, label: "Loaded" },
          }],
        }),
      });
    const line = tasks.line({});

    line.patch(tasks.patch.itemAspect({
      itemId: "t1",
      aspect: "priority",
      value: 2,
    }));
    const localEffect = line.diagnostics().lastEffect;
    assert.deepEqual(line.value().tasks[0], {
      id: "t1",
      metadata: { priority: 2, label: "Loaded" },
    });
    assert.deepEqual(localEffect.locus, {
      kind: "jsonItemAspect",
      itemId: "t1",
      aspect: "priority",
    });
    assert.equal(localEffect.locusProof.locus, "jsonItemAspect");
    assert.equal(localEffect.locusProof.effectLocusDigest.includes("priority"), true);
    assert.equal(localEffect.optimistic.rollback.kind, "exactBranchRestoreAvailable");

    const rollback = line.history().rollbackLastEffect();
    assert.equal(rollback.kind, "rolledBack");
    assert.deepEqual(line.value().tasks[0], {
      id: "t1",
      metadata: { priority: 1, label: "Loaded" },
    });

    line.deliver(signalsMod.resourceDelivery.patch({
      packetId: "pkt-json-priority",
      basisId: null,
      patch: tasks.patch.itemAspect({
        itemId: "t1",
        aspect: "priority",
        value: 3,
      }),
    }));
    const deliveryEffect = line.diagnostics().lastEffect;
    assert.deepEqual(line.value().tasks[0], {
      id: "t1",
      metadata: { priority: 3, label: "Loaded" },
    });
    assert.equal(deliveryEffect.locus.kind, "jsonItemAspect");
    assert.equal(deliveryEffect.locusProof.lensSource, "resource.response.objectItems<T>()(...)");
    assert.deepEqual(
      line.history().verificationPackage().lifecycle.lastEffect.locusProof,
      deliveryEffect.locusProof,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("JSON path aspects deny unsafe segments before declarations can lower", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    assert.throws(
      () =>
        signals.resource.response.jsonPathAspects()({
          polluted: { field: "metadata", path: ["__proto__"] },
        }),
      /rejects unsafe path segment "__proto__"/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("JSON path aspect writes deny missing required paths before effects", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    const response = createJsonPathResponse(signals);
    const tasks = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/missing-json-path")
      .response(response)
      .list({
        load: () => ({
          tasks: [{ id: "t1", metadata: { label: "Loaded" } }],
        }),
      });
    const line = tasks.line({});
    const beforeValue = line.value();
    const beforeEffect = line.diagnostics().lastEffect;

    assert.throws(
      () =>
        line.patch(tasks.patch.itemAspect({
          itemId: "t1",
          aspect: "priority",
          value: 2,
        })),
      /requires existing JSON path segment "priority"/,
    );
    assert.deepEqual(line.value(), beforeValue);
    assert.equal(line.diagnostics().lastEffect, beforeEffect);
  } finally {
    await runtime.cleanup();
  }
});

test("JSON path aspect writes deny non JSON values before effects", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    const response = createJsonPathResponse(signals);
    const tasks = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/non-json-path-value")
      .response(response)
      .list({
        load: () => ({
          tasks: [{ id: "t1", metadata: { priority: 1 } }],
        }),
      });
    const line = tasks.line({});
    const beforeValue = line.value();
    const beforeEffect = line.diagnostics().lastEffect;

    assert.throws(
      () =>
        line.patch(tasks.patch.itemAspect({
          itemId: "t1",
          aspect: "priority",
          value: Number.POSITIVE_INFINITY,
        })),
      /rejects non-finite JSON numbers/,
    );
    assert.deepEqual(line.value(), beforeValue);
    assert.equal(line.diagnostics().lastEffect, beforeEffect);
  } finally {
    await runtime.cleanup();
  }
});

test("JSON path aspect writes cannot change item identity", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    const response = signals.resource.response.objectItems()({
      field: "tasks",
      itemId: (task) => task.identity.value,
      aspects: signals.resource.response.jsonPathAspects()({
        identityValue: { field: "identity", path: ["value"] },
      }),
    });
    const tasks = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/json-identity")
      .response(response)
      .list({
        load: () => ({
          tasks: [{
            identity: { value: "t1" },
            metadata: { priority: 1 },
          }],
        }),
      });
    const line = tasks.line({});
    const beforeValue = line.value();
    const beforeEffect = line.diagnostics().lastEffect;

    assert.throws(
      () =>
        line.patch(tasks.patch.itemAspect({
          itemId: "t1",
          aspect: "identityValue",
          value: "t2",
        })),
      /resourcePatch\.itemAspect\(\.\.\.\) to preserve item identity "t1"/,
    );
    assert.deepEqual(line.value(), beforeValue);
    assert.equal(line.diagnostics().lastEffect, beforeEffect);
  } finally {
    await runtime.cleanup();
  }
});

function createJsonPathResponse(signals) {
  return signals.resource.response.objectItems()({
    field: "tasks",
    itemId: (task) => task.id,
    aspects: signals.resource.response.jsonPathAspects()({
      priority: { field: "metadata", path: ["priority"] },
    }),
  });
}
