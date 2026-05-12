import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";

test("response contracts lower compiled lens proof into branch-native effect loci", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const response = signals.resource.response.objectItems()({
      field: "tasks",
      itemId: (task) => task.id,
      aspects: signals.resource.response.objectAspects()({
        title: "title",
      }),
      summaries: signalsMod.resourceValueSummaries({
        total: {
          read: (value) => value.total,
          write: (value, total) => ({ ...value, total }),
        },
      }),
    });
    const tasks = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/tasks")
      .response(response)
      .list({
        load: () => ({
          tasks: [{ id: "t1", title: "First" }],
          cursor: "next",
          total: 1,
        }),
      });

    const line = tasks.line({});
    line.patch(
      tasks.patch.itemAspect({
        itemId: "t1",
        aspect: "title",
        value: "Renamed",
      }),
    );

    const effect = line.diagnostics().lastEffect;
    assert.deepEqual(effect.locus, { kind: "itemAspect", itemId: "t1", aspect: "title" });
    assert.equal(effect.locusProof.lensSource, "resource.response.objectItems<T>()(...)");
    assert.equal(effect.locusProof.topology, "objectItems");
    assert.equal(effect.locusProof.itemField, "tasks");
    assert.equal(effect.locusProof.locus, "itemAspect");
    assert.equal(effect.locusProof.patchScope, "aspect");
    assert.equal(effect.locusProof.aspect, "title");
    assert.equal(effect.locusProof.declarationDigest, response.lensProof.declarationDigest);
    assert.equal(effect.locusProof.capabilityDigest, response.lensProof.capabilityDigest);
    assert.equal(effect.locusProof.compiledLensDigest, response.lensProof.compiledLensDigest);
    assert.equal(effect.locusProof.capabilityRowDigest.includes("itemAspect"), true);
    assert.equal(effect.locusProof.effectLocusDigest.includes("title"), true);
    assert.equal(effect.counters.responseLensBreadth, 1);
    assert.equal(effect.counters.effectLocusBreadth, 1);
    assert.deepEqual(
      line.history().verificationPackage().lifecycle.lastEffect.locusProof,
      effect.locusProof,
    );

    line.patch(
      tasks.patch.summary({
        summary: "total",
        value: 2,
      }),
    );
    const summaryEffect = line.diagnostics().lastEffect;
    assert.deepEqual(summaryEffect.locus, { kind: "summary", summary: "total" });
    assert.equal(summaryEffect.locusProof.locus, "summary");
    assert.equal(summaryEffect.locusProof.patchScope, "summary");
    assert.equal(summaryEffect.locusProof.summary, "total");
    assert.equal(summaryEffect.locusProof.summaryPatchScope, "line");
    assert.equal(summaryEffect.counters.responseLensBreadth, 1);
    assert.deepEqual(line.value(), {
      tasks: [{ id: "t1", title: "Renamed" }],
      cursor: "next",
      total: 2,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("response summary effect loci preserve line and page-window scope proof", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const response = signals.resource.response.collection({
      itemId: (task) => task.id,
      items: (value) => value.items,
      replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      summaries: signalsMod.resourceValueSummaries.pageWindow({
        visibleCount: {
          read: (value) => value.visibleCount,
          write: (value, visibleCount) => ({ ...value, visibleCount }),
        },
      }),
    });
    const feed = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/feed")
      .response(response)
      .paged({
        accumulatePage: (existing, next) => ({
          items: [...existing.items, ...next.items],
          visibleCount: next.visibleCount,
        }),
        load: () => ({
          items: [{ id: "page:1", title: "First" }],
          visibleCount: 1,
        }),
      });

    const line = feed.line({});
    line.deliver(
      signalsMod.resourceDelivery.patch({
        packetId: "pkt-visible-count",
        basisId: null,
        patch: signalsMod.resourcePatch.summary({
          summary: "visibleCount",
          value: 2,
        }),
      }),
    );

    const effectProof = line.diagnostics().lastEffect.locusProof;
    assert.equal(effectProof.lensSource, "resource.response.collection(...)");
    assert.equal(effectProof.topology, "customCollection");
    assert.equal(effectProof.locus, "summary");
    assert.equal(effectProof.patchScope, "summary");
    assert.equal(effectProof.summary, "visibleCount");
    assert.equal(effectProof.summaryPatchScope, "pageWindow");
    assert.equal(effectProof.capabilityRowDigest.includes("pageWindow"), true);
    assert.deepEqual(line.value(), {
      items: [{ id: "page:1", title: "First" }],
      visibleCount: 2,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("response JSON object aspects lower to JSON item-aspect effect loci", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const response = signals.resource.response.objectItems()({
      field: "tasks",
      itemId: (task) => task.id,
      aspects: signals.resource.response.jsonObjectAspects()({
        metadata: "metadata",
      }),
    });
    assert.deepEqual(response.lensProof.jsonAspectNames, ["metadata"]);
    assert.equal(response.lensProof.capabilityRows.some(
      (row) => row.locus === "jsonItemAspect",
    ), true);

    const tasks = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/tasks")
      .response(response)
      .list({
        load: () => ({
          tasks: [{ id: "t1", metadata: { priority: 1 } }],
        }),
      });
    const line = tasks.line({});
    line.patch(
      tasks.patch.itemAspect({
        itemId: "t1",
        aspect: "metadata",
        value: { priority: 2 },
      }),
    );

    const effect = line.diagnostics().lastEffect;
    assert.deepEqual(effect.locus, {
      kind: "jsonItemAspect",
      itemId: "t1",
      aspect: "metadata",
    });
    assert.equal(effect.locusProof.locus, "jsonItemAspect");
    assert.equal(effect.locusProof.patchScope, "aspect");
    assert.deepEqual(line.value().tasks[0].metadata, { priority: 2 });

    const ordinaryResponse = signals.resource.response.objectItems()({
      field: "tasks", itemId: (task) => task.id,
      aspects: signals.resource.response.objectAspects()({ metadata: "metadata" }),
    });
    const ordinaryTasks = signals.api({ effects: signals.resource.effects.pessimistic() })
      .url("/ordinary-tasks").response(ordinaryResponse)
      .list({ load: () => ({ tasks: [{ id: "t1", metadata: { priority: 1 } }] }) });
    const ordinaryLine = ordinaryTasks.line({});
    ordinaryLine.patch(signalsMod.resourcePatch.itemAspect({
      itemId: "t1", aspect: "metadata",
      aspectLocus: "jsonItemAspect", value: { priority: 2 },
    }));
    assert.equal(ordinaryLine.diagnostics().lastEffect.locus.kind, "itemAspect");
  } finally {
    await runtime.cleanup();
  }
});

test("direct array custom collection and paged responses preserve topology proof parity", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    const arrayResponse = signals.resource.response.array({
      itemId: (task) => task.id,
    });
    const directTasks = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/direct")
      .response(arrayResponse)
      .list({
        load: () => [{ id: "direct:1", title: "First" }],
      });
    const directLine = directTasks.line({});
    directLine.patch(
      directTasks.patch.item({
        itemId: "direct:1",
        nextItem: { id: "direct:1", title: "Direct" },
      }),
    );
    assert.deepEqual(directLine.diagnostics().lastEffect.locus, {
      kind: "membership",
      itemId: "direct:1",
    });
    assert.equal(
      directLine.diagnostics().lastEffect.locusProof.topology,
      "directArray",
    );
    assert.equal(directLine.diagnostics().lastEffect.locusProof.locus, "membership");
    assert.equal(directLine.diagnostics().lastEffect.locusProof.patchScope, "item");

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
    }).url("/connection")
      .response(customResponse)
      .list({
        load: () => ({
          edges: [{ node: { id: "node:1", title: "First" } }],
        }),
      });
    const connectionLine = connection.line({});
    connectionLine.patch(
      connection.patch.item({
        itemId: "node:1",
        nextItem: { id: "node:1", title: "Custom" },
      }),
    );
    assert.deepEqual(connectionLine.diagnostics().lastEffect.locus, {
      kind: "membership",
      itemId: "node:1",
    });
    assert.equal(
      connectionLine.diagnostics().lastEffect.locusProof.topology,
      "customCollection",
    );

    const smuggledTopology = signals.resource.response.collection({
      topology: "objectItems",
      itemField: "tasks",
      itemId: (task) => task.id,
      items: (value) => value.tasks,
      replaceItems: (value, nextItems) => ({ ...value, tasks: [...nextItems] }),
    });
    assert.equal(smuggledTopology.lensProof.topology, "customCollection");
    assert.equal(smuggledTopology.lensProof.itemField, null);

    const paged = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/paged")
      .response(arrayResponse)
      .paged({
        accumulatePage: (existing, next) => [...existing, ...next],
        load: () => [{ id: "page:1", title: "First" }],
      });
    const pagedLine = paged.line({});
    pagedLine.patch(
      paged.patch.item({
        itemId: "page:1",
        nextItem: { id: "page:1", title: "Paged" },
      }),
    );
    assert.equal(
      pagedLine.diagnostics().lastEffect.locusProof.topology,
      "directArray",
    );
  } finally {
    await runtime.cleanup();
  }
});


test("response topology denials preserve value diagnostics and effect proof", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    assert.throws(
      () =>
        signalsMod.resourceCollectionShape({
          items: (value) => value,
          replaceItems: (_value, nextItems) => [...nextItems],
          responseLensProof: {},
        }),
      /does not accept responseLensProof/,
    );

    const badResponse = signals.resource.response.collection({
      itemId: (task) => task.id,
      items: () => ({ not: "items" }),
      replaceItems: (value) => value,
    });
    const tasks = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/bad")
      .response(badResponse)
      .list({
        load: () => ({ tasks: [{ id: "t1" }] }),
      });
    const line = tasks.line({});
    const beforeValue = line.value();
    const beforeEffect = line.diagnostics().lastEffect;

    assert.throws(
      () =>
        line.patch(
          tasks.patch.item({
            itemId: "t1",
            nextItem: { id: "t1" },
          }),
        ),
      /requires items\(value\) to produce an array/,
    );
    assert.deepEqual(line.value(), beforeValue);
    assert.equal(line.diagnostics().lastEffect, beforeEffect);

    const pageWindowResponse = signals.resource.response.collection({
      itemId: (task) => task.id,
      items: (value) => value.items,
      replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      summaries: signalsMod.resourceValueSummaries.pageWindow({
        visibleCount: {
          read: (value) => value.visibleCount,
          write: (value, visibleCount) => ({ ...value, visibleCount }),
        },
      }),
    });
    const pageWindowList = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/page-window-list")
      .response(pageWindowResponse)
      .list({
        load: () => ({
          items: [{ id: "window:1", title: "First" }],
          visibleCount: 1,
        }),
      });
    const pageWindowLine = pageWindowList.line({});
    const beforePageWindowValue = pageWindowLine.value();
    const beforePageWindowEffect = pageWindowLine.diagnostics().lastEffect;
    assert.deepEqual(pageWindowLine.reconciliation().summaryNames, []);
    assert.throws(
      () =>
        pageWindowLine.patch(
          signalsMod.resourcePatch.summary({
            summary: "visibleCount",
            value: 2,
          }),
        ),
      /do not admit resourceValueSummaries\.pageWindow/,
    );
    assert.deepEqual(pageWindowLine.value(), beforePageWindowValue);
    assert.equal(pageWindowLine.diagnostics().lastEffect, beforePageWindowEffect);
  } finally {
    await runtime.cleanup();
  }
});
