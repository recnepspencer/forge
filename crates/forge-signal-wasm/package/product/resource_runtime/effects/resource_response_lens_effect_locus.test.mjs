import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";

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
    assert.deepEqual(effect.locus, {
      kind: "itemAspect",
      itemId: "t1",
      aspect: "title",
    });
    assert.deepEqual(effect.locusProof, {
      version: "resource-effect-locus-proof-v1",
      lensVersion: "resource-response-lens-proof-v1",
      lensSource: "resource.response.objectItems<T>()(...)",
      topology: "objectItems",
      itemField: "tasks",
      locus: "itemAspect",
      patchScope: "aspect",
      aspect: "title",
      summary: null,
      summaryPatchScope: null,
      proofBreadth: 1,
    });
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
    assert.deepEqual(summaryEffect.locus, {
      kind: "summary",
      summary: "total",
    });
    assert.deepEqual(summaryEffect.locusProof, {
      version: "resource-effect-locus-proof-v1",
      lensVersion: "resource-response-lens-proof-v1",
      lensSource: "resource.response.objectItems<T>()(...)",
      topology: "objectItems",
      itemField: "tasks",
      locus: "summary",
      patchScope: "summary",
      aspect: null,
      summary: "total",
      summaryPatchScope: "line",
      proofBreadth: 1,
    });
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

    assert.deepEqual(line.diagnostics().lastEffect.locusProof, {
      version: "resource-effect-locus-proof-v1",
      lensVersion: "resource-response-lens-proof-v1",
      lensSource: "resource.response.collection(...)",
      topology: "customCollection",
      itemField: null,
      locus: "summary",
      patchScope: "summary",
      aspect: null,
      summary: "visibleCount",
      summaryPatchScope: "pageWindow",
      proofBreadth: 1,
    });
    assert.deepEqual(line.value(), {
      items: [{ id: "page:1", title: "First" }],
      visibleCount: 2,
    });
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
    assert.equal(
      directLine.diagnostics().lastEffect.locusProof.topology,
      "directArray",
    );
    assert.equal(directLine.diagnostics().lastEffect.locusProof.locus, "membership");

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
      /requires a compiled response lens proof/,
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

    const response = signals.resource.response.array({
      itemId: (task) => task.id,
    });
    const summaryBlocked = signals.resource.collection({
      params: signalsMod.resourceParams(),
      effects: signals.resource.effects.pessimistic(),
      normalizeParams: ({ workspaceId }) =>
        signalsMod.resourceParamIdentity({ workspaceId }, workspaceId),
      itemIdentity: (task) => task.id,
      reconcile: signalsMod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        summaries: signalsMod.resourceValueSummaries({
          total: {
            read: (value) => value.total,
            write: (value, total) => ({ ...value, total }),
          },
        }),
        responseLensProof: response.lensProof,
      }),
      load: () => ({
        items: [{ id: "summary:1", title: "First" }],
        total: 1,
      }),
    });
    const summaryLine = summaryBlocked.line({ workspaceId: "demo" });
    const beforeSummaryValue = summaryLine.value();
    const beforeSummaryEffect = summaryLine.diagnostics().lastEffect;
    assert.throws(
      () =>
        summaryLine.patch(
          signalsMod.resourcePatch.summary({
            summary: "total",
            value: 2,
          }),
        ),
      /cannot lower effect locus "summary"/,
    );
    assert.deepEqual(summaryLine.value(), beforeSummaryValue);
    assert.equal(summaryLine.diagnostics().lastEffect, beforeSummaryEffect);

    assert.throws(
      () =>
        summaryLine.deliver(
          signalsMod.resourceDelivery.patch({
            packetId: "pkt-summary-denied",
            basisId: null,
            patch: signalsMod.resourcePatch.summary({
              summary: "total",
              value: 3,
            }),
          }),
        ),
      /cannot lower effect locus "summary"/,
    );
    assert.deepEqual(summaryLine.value(), beforeSummaryValue);
    assert.equal(summaryLine.diagnostics().lastEffect, beforeSummaryEffect);

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
