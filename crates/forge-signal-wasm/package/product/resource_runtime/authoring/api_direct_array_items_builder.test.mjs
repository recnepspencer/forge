import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";
import { normalizeRouteLineArtifact } from "./api_route/route_line_artifact_proof.mjs";

test("api.url(...).items(...).aspect(...).summary(...).list(...) lowers direct arrays into one raw collection truth with owned aspect and summary helpers", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const tasks = signals.api({}).url("/workspaces/:workspaceId/tasks")
      .items((item) => item.id)
      .aspect(
        "title",
        (item) => item.title,
        (item, title) => ({ ...item, title }),
      )
      .summary(
        "count",
        (value) => value.length,
        (value, count) => value.slice(0, count),
      )
      .list({
        load: ({ workspaceId }) => [
          { id: `${workspaceId}:1`, title: "First" },
        ],
      });
    const rawTasks = signals.resource.collection({
      params: signalsMod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        signalsMod.resourceParamIdentity(
          { workspaceId },
          `/workspaces/${encodeURIComponent(String(workspaceId))}/tasks`,
        ),
      itemIdentity: (item) => item.id,
      reconcile: signalsMod.resourceCollectionShape({
        items: (value) => value,
        replaceItems: (_value, nextItems) => [...nextItems],
        aspects: signalsMod.resourceItemAspects({
          title: {
            read: (item) => item.title,
            write: (item, title) => ({ ...item, title }),
          },
        }),
        summaries: signalsMod.resourceValueSummaries({
          count: {
            read: (value) => value.length,
            write: (value, count) => value.slice(0, count),
          },
        }),
      }),
      load: ({ workspaceId }) => [
        { id: `${workspaceId}:1`, title: "First" },
      ],
    });

    assert.deepEqual(
      normalizeRouteLineArtifact(tasks.line({ workspaceId: "demo" })),
      normalizeRouteLineArtifact(rawTasks.line({ workspaceId: "demo" })),
    );
    assert.equal(typeof tasks.patch.item, "function");
    assert.equal(typeof tasks.delivery.item, "function");
    assert.equal(typeof tasks.patch.itemAspect, "function");
    assert.equal(typeof tasks.delivery.summary, "function");

    const line = tasks.line({ workspaceId: "demo" });
    const aspectResult = line.patch(
      tasks.patch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Patched",
      }),
    );
    assert.deepEqual(aspectResult, {
      kind: "narrowed",
      scope: "aspect",
      itemId: "demo:1",
      aspect: "title",
    });
    const deliveryResult = line.deliver(
      tasks.delivery.summary({
        packetId: "pkt-summary",
        basisId: null,
        nextBasisId: "basis-1",
        summary: "count",
        value: 1,
      }),
    );
    assert.deepEqual(deliveryResult, {
      kind: "applied",
      deliveryKind: "patch",
      scope: "summary",
      packetId: "pkt-summary",
      basisId: null,
      nextBasisId: "basis-1",
      supersededOperation: null,
    });
    assert.deepEqual(line.value(), [{ id: "demo:1", title: "Patched" }]);
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).items(...).aspect(...).pageWindowSummary(...).paged(...) lowers direct arrays into one raw paged truth with page-window summary helpers", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const tasks = signals.api({}).url("/workspaces/:workspaceId/tasks")
      .items((item) => item.id)
      .aspect(
        "title",
        (item) => item.title,
        (item, title) => ({ ...item, title }),
      )
      .pageWindowSummary(
        "count",
        (value) => value.length,
        (value, count) => value.slice(0, count),
      )
      .paged({
        accumulatePage: (existing, next) => [...existing, ...next],
        load: ({ workspaceId }) => [
          { id: `${workspaceId}:1`, title: "First" },
        ],
      });
    const rawTasks = signals.resource.paged({
      params: signalsMod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        signalsMod.resourceParamIdentity(
          { workspaceId },
          `/workspaces/${encodeURIComponent(String(workspaceId))}/tasks`,
        ),
      itemIdentity: (item) => item.id,
      reconcile: signalsMod.resourceCollectionShape({
        items: (value) => value,
        replaceItems: (_value, nextItems) => [...nextItems],
        aspects: signalsMod.resourceItemAspects({
          title: {
            read: (item) => item.title,
            write: (item, title) => ({ ...item, title }),
          },
        }),
        summaries: signalsMod.resourceValueSummaries.pageWindow({
          count: {
            read: (value) => value.length,
            write: (value, count) => value.slice(0, count),
          },
        }),
      }),
      accumulatePage: (existing, next) => [...existing, ...next],
      load: ({ workspaceId }) => [
        { id: `${workspaceId}:1`, title: "First" },
      ],
    });

    assert.deepEqual(
      normalizeRouteLineArtifact(tasks.line({ workspaceId: "demo" })),
      normalizeRouteLineArtifact(rawTasks.line({ workspaceId: "demo" })),
    );
    assert.equal(typeof tasks.patch.item, "function");
    assert.equal(typeof tasks.delivery.item, "function");
    assert.equal(typeof tasks.patch.summary, "function");
    assert.equal(typeof tasks.delivery.summary, "function");

    const line = tasks.line({ workspaceId: "demo" });
    const patchResult = line.patch(
      tasks.patch.summary({
        summary: "count",
        value: 1,
      }),
    );
    assert.deepEqual(patchResult, {
      kind: "narrowed",
      scope: "summary",
      itemId: null,
      aspect: null,
      summary: "count",
    });
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).items(...).reconcile(...).aspect(...).summary(...).list(...) lowers envelope-shaped values into one raw collection truth", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const catalog = signals.api({}).url("/workspaces/:workspaceId/catalog")
      .items((item) => item.id)
      .reconcile(
        (value) => value.items,
        (value, nextItems) => ({ ...value, items: [...nextItems] }),
      )
      .aspect(
        "title",
        (item) => item.title,
        (item, title) => ({ ...item, title }),
      )
      .summary(
        "total",
        (value) => value.total,
        (value, total) => ({ ...value, total }),
      )
      .list({
        load: ({ workspaceId }) => ({
          items: [{ id: `${workspaceId}:1`, title: "First" }],
          total: 1,
        }),
      });
    const rawCatalog = signals.resource.collection({
      params: signalsMod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        signalsMod.resourceParamIdentity(
          { workspaceId },
          `/workspaces/${encodeURIComponent(String(workspaceId))}/catalog`,
        ),
      itemIdentity: (item) => item.id,
      reconcile: signalsMod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        aspects: signalsMod.resourceItemAspects({
          title: {
            read: (item) => item.title,
            write: (item, title) => ({ ...item, title }),
          },
        }),
        summaries: signalsMod.resourceValueSummaries({
          total: {
            read: (value) => value.total,
            write: (value, total) => ({ ...value, total }),
          },
        }),
      }),
      load: ({ workspaceId }) => ({
        items: [{ id: `${workspaceId}:1`, title: "First" }],
        total: 1,
      }),
    });

    assert.deepEqual(
      normalizeRouteLineArtifact(catalog.line({ workspaceId: "demo" })),
      normalizeRouteLineArtifact(rawCatalog.line({ workspaceId: "demo" })),
    );
    assert.equal(typeof catalog.patch.itemAspect, "function");
    assert.equal(typeof catalog.delivery.summary, "function");

    const line = catalog.line({ workspaceId: "demo" });
    const patchResult = line.patch(
      catalog.patch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Updated",
      }),
    );
    assert.deepEqual(patchResult, {
      kind: "narrowed",
      scope: "aspect",
      itemId: "demo:1",
      aspect: "title",
    });
    assert.deepEqual(line.value(), {
      items: [{ id: "demo:1", title: "Updated" }],
      total: 1,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).items(...).reconcile(...).pageWindowSummary(...).paged(...) lowers envelope-shaped paged values into one raw truth", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const catalog = signals.api({}).url("/workspaces/:workspaceId/catalog-pages")
      .items((item) => item.id)
      .reconcile(
        (value) => value.items,
        (value, nextItems) => ({ ...value, items: [...nextItems] }),
      )
      .pageWindowSummary(
        "total",
        (value) => value.total,
        (value, total) => ({ ...value, total }),
      )
      .paged({
        accumulatePage: (existing, next) => ({
          items: [...existing.items, ...next.items],
          total: next.total,
        }),
        load: ({ workspaceId }) => ({
          items: [{ id: `${workspaceId}:1`, title: "First" }],
          total: 1,
        }),
      });
    const rawCatalog = signals.resource.paged({
      params: signalsMod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        signalsMod.resourceParamIdentity(
          { workspaceId },
          `/workspaces/${encodeURIComponent(String(workspaceId))}/catalog-pages`,
        ),
      itemIdentity: (item) => item.id,
      reconcile: signalsMod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        summaries: signalsMod.resourceValueSummaries.pageWindow({
          total: {
            read: (value) => value.total,
            write: (value, total) => ({ ...value, total }),
          },
        }),
      }),
      accumulatePage: (existing, next) => ({
        items: [...existing.items, ...next.items],
        total: next.total,
      }),
      load: ({ workspaceId }) => ({
        items: [{ id: `${workspaceId}:1`, title: "First" }],
        total: 1,
      }),
    });

    assert.deepEqual(
      normalizeRouteLineArtifact(catalog.line({ workspaceId: "demo" })),
      normalizeRouteLineArtifact(rawCatalog.line({ workspaceId: "demo" })),
    );
    assert.equal(typeof catalog.patch.summary, "function");
    assert.equal(typeof catalog.delivery.summary, "function");
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).items(...) owns direct-array and custom reconcile authoring boundaries", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    assert.throws(
      () =>
        runtime.signals.api({}).url("/tasks")
          .items((item) => item.id)
          .summary("count", (value) => value.length, (value, count) =>
            value.slice(0, count))
          .reconcile(
            (value) => value.items,
            (value, nextItems) => ({ ...value, items: [...nextItems] }),
          ),
      /must be declared before summary/,
    );
    assert.throws(
      () => {
        const brokenCatalog = runtime.signals.api({}).url("/tasks")
          .items((item) => item.id)
          .reconcile(
            (value) => value.items,
            (value, nextItems) => ({ ...value, items: [...nextItems] }),
          )
          .list({ load: () => ({ items: { wrong: true } }) });

        brokenCatalog.line({}).patch(
          brokenCatalog.patch.item({
            itemId: "t1",
            nextItem: { id: "t1" },
          }),
        );
      },
      /requires items\(value\) to return an array/,
    );
    assert.throws(
      () =>
        runtime.signals.api({}).url("/tasks")
          .items((item) => item.id)
          .list({
            itemIdentity: (item) => item.id,
            load: () => [{ id: "t1" }],
          }),
      /owns itemIdentity/,
    );
    assert.throws(
      () =>
        runtime.signals.api({}).url("/tasks")
          .items((item) => item.id)
          .list({
            reconcile: runtime.signalsMod.resourceCollectionShape({
              items: (value) => value,
              replaceItems: (_value, nextItems) => [...nextItems],
            }),
            load: () => [{ id: "t1" }],
          }),
      /owns reconcile/,
    );
    assert.throws(
      () =>
        runtime.signals.api({}).url("/tasks")
          .items((item) => item.id)
          .aspect("title", (item) => item.title, (item, title) => ({
            ...item,
            title,
          }))
          .aspect("title", (item) => item.title, (item, title) => ({
            ...item,
            title,
          })),
      /already exists/,
    );
    assert.throws(
      () =>
        runtime.signals.api({}).url("/tasks")
          .items((item) => item.id)
          .summary("count", (value) => value.length, (value, count) =>
            value.slice(0, count))
          .pageWindowSummary("windowCount", (value) => value.length, (value, count) =>
            value.slice(0, count)),
      /cannot mix summary/,
    );
    const brokenValue = runtime.signals.api({}).url("/tasks")
      .items((item) => item.id)
      .list({
        load: () => ({
          items: [{ id: "t1" }],
        }),
      });

    assert.throws(
      () =>
        brokenValue.line({}).patch(
          brokenValue.patch.item({
            itemId: "t1",
            nextItem: { id: "t1" },
          }),
        ),
      /requires list\/paged values to stay direct arrays/,
    );
  } finally {
    await runtime.cleanup();
  }
});
