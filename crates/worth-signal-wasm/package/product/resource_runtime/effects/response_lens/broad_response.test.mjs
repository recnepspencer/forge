import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";

function assertBroadResponseEffect(effect, expected) {
  assert.deepEqual(effect.locus, { kind: "broadResponse" });
  assert.equal(effect.locusProof.version, "resource-effect-locus-proof-v1");
  assert.equal(effect.locusProof.lensVersion, "resource-response-lens-proof-v1");
  assert.equal(effect.locusProof.lensSource, expected.lensSource);
  assert.equal(effect.locusProof.topology, expected.topology);
  assert.equal(effect.locusProof.itemField, expected.itemField);
  assert.equal(effect.locusProof.locus, "broadResponse");
  assert.equal(effect.locusProof.patchScope, "line");
  assert.equal(effect.locusProof.parityDigest, expected.parityDigest);
  assert.equal(effect.locusProof.compileBoundaryDigest, expected.compileBoundaryDigest);
  assert.equal(effect.locusProof.capabilityRowDigest.includes("broadResponse"), true);
  assert.equal(effect.locusProof.effectLocusDigest.includes(effect.locusProof.compiledLensDigest), true);
  assert.equal(effect.counters.responseLensBreadth, 1);
  assert.equal(effect.counters.effectLocusBreadth, 1);
}

test("response broad replacements lower through compiled lens proof", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const response = signals.resource.response.objectItems()({
      field: "tasks",
      itemId: (task) => task.id,
    });
    const tasks = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/broad-response")
      .response(response)
      .list({
        load: () => ({
          tasks: [{ id: "t1", title: "First" }],
          total: 1,
        }),
      });

    const line = tasks.line({});
    await line.patch(
      signalsMod.resourcePatch.replace({
        tasks: [{ id: "t2", title: "Replacement" }],
        total: 1,
      }),
    );

    const localEffect = line.diagnostics().lastEffect;
    assert.deepEqual(line.value(), {
      tasks: [{ id: "t2", title: "Replacement" }],
      total: 1,
    });
    assertBroadResponseEffect(localEffect, {
      lensSource: "resource.response.objectItems<T>()(...)",
      topology: "objectItems",
      itemField: "tasks",
      parityDigest: response.lensProof.parityDigest,
      compileBoundaryDigest: response.lensProof.compileBoundaryDigest,
    });
    assert.deepEqual(
      line.history().verificationPackage().lifecycle.lastEffect.locusProof,
      localEffect.locusProof,
    );

    line.deliver(
      signalsMod.resourceDelivery.replace({
        packetId: "pkt-broad-replace",
        basisId: null,
        nextValue: {
          tasks: [{ id: "t3", title: "Delivered" }],
          total: 1,
        },
      }),
    );

    const deliveryEffect = line.diagnostics().lastEffect;
    assert.equal(deliveryEffect.provenance, "deliveredReplace");
    assert.deepEqual(line.value(), {
      tasks: [{ id: "t3", title: "Delivered" }],
      total: 1,
    });
    assertBroadResponseEffect(deliveryEffect, {
      lensSource: "resource.response.objectItems<T>()(...)",
      topology: "objectItems",
      itemField: "tasks",
      parityDigest: response.lensProof.parityDigest,
      compileBoundaryDigest: response.lensProof.compileBoundaryDigest,
    });
    assert.deepEqual(deliveryEffect.locusProof, localEffect.locusProof);
    assert.deepEqual(
      line.history().verificationPackage().lifecycle.lastEffect.locusProof,
      deliveryEffect.locusProof,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("broad response replacement proof preserves topology parity", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const arrayResponse = signals.resource.response.array({
      itemId: (task) => task.id,
    });

    const directTasks = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/direct-broad")
      .response(arrayResponse)
      .list({
        load: () => [{ id: "direct:1", title: "First" }],
      });
    const directLine = directTasks.line({});
    directLine.patch(
      signalsMod.resourcePatch.replace([
        { id: "direct:2", title: "Replacement" },
      ]),
    );
    assert.deepEqual(directLine.value(), [
      { id: "direct:2", title: "Replacement" },
    ]);
    assertBroadResponseEffect(directLine.diagnostics().lastEffect, {
      lensSource: "resource.response.array(...)",
      topology: "directArray",
      itemField: null,
      parityDigest: arrayResponse.lensProof.parityDigest,
      compileBoundaryDigest: arrayResponse.lensProof.compileBoundaryDigest,
    });

    const customResponse = signals.resource.response.collection({
      itemId: (task) => task.id,
      items: (value) => value.edges.map((edge) => edge.node),
      replaceItems: (value, nextItems) => ({
        ...value,
        edges: nextItems.map((node) => ({ node })),
      }),
    });
    const connection = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/connection-broad")
      .response(customResponse)
      .list({
        load: () => ({
          edges: [{ node: { id: "node:1", title: "First" } }],
        }),
      });
    const connectionLine = connection.line({});
    connectionLine.patch(
      signalsMod.resourcePatch.replace({
        edges: [{ node: { id: "node:2", title: "Replacement" } }],
      }),
    );
    assert.deepEqual(connectionLine.value(), {
      edges: [{ node: { id: "node:2", title: "Replacement" } }],
    });
    assertBroadResponseEffect(connectionLine.diagnostics().lastEffect, {
      lensSource: "resource.response.collection(...)",
      topology: "customCollection",
      itemField: null,
      parityDigest: customResponse.lensProof.parityDigest,
      compileBoundaryDigest: customResponse.lensProof.compileBoundaryDigest,
    });

    const paged = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/paged-broad")
      .response(arrayResponse)
      .paged({
        accumulatePage: (existing, next) => [...existing, ...next],
        load: () => [{ id: "page:1", title: "First" }],
      });
    const pagedLine = paged.line({});
    pagedLine.patch(
      signalsMod.resourcePatch.replace([
        { id: "page:2", title: "Replacement" },
      ]),
    );
    assert.deepEqual(pagedLine.value(), [
      { id: "page:2", title: "Replacement" },
    ]);
    assertBroadResponseEffect(pagedLine.diagnostics().lastEffect, {
      lensSource: "resource.response.array(...)",
      topology: "directArray",
      itemField: null,
      parityDigest: arrayResponse.lensProof.parityDigest,
      compileBoundaryDigest: arrayResponse.lensProof.compileBoundaryDigest,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("detail response broad replacements lower through compiled lens proof", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const response = signals.resource.response.detail()();
    const user = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/users/:userId")
      .response(response)
      .detail({
        load: ({ userId }) => ({ id: userId, name: "First" }),
      });

    const line = user.line({ userId: "u1" });
    assert.equal("patch" in line, true);
    await line.patch(
      signalsMod.resourcePatch.replace({ id: "u1", name: "Renamed" }),
    );

    const localEffect = line.diagnostics().lastEffect;
    assert.deepEqual(line.value(), { id: "u1", name: "Renamed" });
    assert.deepEqual(localEffect.locus, { kind: "detailResponse" });
    assert.equal(localEffect.locusProof.lensSource, "resource.response.detail<T>()");
    assert.equal(localEffect.locusProof.topology, "detail");
    assert.equal(localEffect.locusProof.locus, "detailResponse");
    assert.equal(localEffect.locusProof.patchScope, "line");
    assert.equal(localEffect.locusProof.compiledLensDigest, response.lensProof.compiledLensDigest);
    assert.equal(localEffect.counters.responseLensBreadth, 1);

    line.deliver(
      signalsMod.resourceDelivery.replace({
        packetId: "pkt-detail-replace",
        basisId: null,
        nextValue: { id: "u1", name: "Delivered" },
      }),
    );

    const deliveryEffect = line.diagnostics().lastEffect;
    assert.deepEqual(line.value(), { id: "u1", name: "Delivered" });
    assert.deepEqual(deliveryEffect.locus, { kind: "detailResponse" });
    assert.equal(deliveryEffect.locusProof.locus, "detailResponse");
    assert.deepEqual(
      line.history().verificationPackage().lifecycle.lastEffect.locusProof,
      deliveryEffect.locusProof,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("summary response broad replacements lower through compiled lens proof", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const response = signals.resource.response.summary()();
    const totals = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/task-summary")
      .response(response)
      .detail({
        load: () => ({ open: 1, closed: 0 }),
      });

    const line = totals.line({});
    await line.patch(
      signalsMod.resourcePatch.replace({ open: 2, closed: 1 }),
    );

    const localEffect = line.diagnostics().lastEffect;
    assert.deepEqual(line.value(), { open: 2, closed: 1 });
    assert.deepEqual(localEffect.locus, { kind: "summaryResponse" });
    assert.equal(localEffect.locusProof.lensSource, "resource.response.summary<T>()");
    assert.equal(localEffect.locusProof.topology, "summary");
    assert.equal(localEffect.locusProof.locus, "summaryResponse");
    assert.equal(localEffect.locusProof.patchScope, "line");
    assert.equal(localEffect.locusProof.compiledLensDigest, response.lensProof.compiledLensDigest);
    assert.equal(localEffect.counters.responseLensBreadth, 1);
    assert.equal(localEffect.counters.effectLocusBreadth, 1);

    line.deliver(
      signalsMod.resourceDelivery.replace({
        packetId: "pkt-summary-replace",
        basisId: null,
        nextValue: { open: 3, closed: 1 },
      }),
    );

    const deliveryEffect = line.diagnostics().lastEffect;
    assert.deepEqual(line.value(), { open: 3, closed: 1 });
    assert.deepEqual(deliveryEffect.locus, { kind: "summaryResponse" });
    assert.equal(deliveryEffect.locusProof.locus, "summaryResponse");
    assert.deepEqual(
      line.history().verificationPackage().lifecycle.lastEffect.locusProof,
      deliveryEffect.locusProof,
    );
  } finally {
    await runtime.cleanup();
  }
});
