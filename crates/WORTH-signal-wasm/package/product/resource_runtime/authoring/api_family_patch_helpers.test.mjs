import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";

test("api.url(...).list(...) families own typed patch helpers without changing patch semantics", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const catalog = signals.api({}).url("/workspaces/:workspaceId/catalog").list({
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

    assert.equal(typeof catalog.patch.replace, "function");
    assert.equal(typeof catalog.patch.item, "function");
    assert.equal(typeof catalog.patch.itemAspect, "function");
    assert.equal(typeof catalog.patch.summary, "function");
    assert.deepEqual(
      catalog.patch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Updated",
      }),
      signalsMod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Updated",
      }),
    );

    const line = catalog.line({ workspaceId: "demo" });
    const result = line.patch(
      catalog.patch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Updated",
      }),
    );

    assert.deepEqual(result, {
      kind: "narrowed",
      scope: "aspect",
      itemId: "demo:1",
      aspect: "title",
      field: null,
    });
    assert.deepEqual(line.value(), {
      items: [{ id: "demo:1", title: "Updated" }],
      total: 1,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).paged(...) patch helpers do not overclaim line-scoped summary admission", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const feed = signals.api({}).url("/workspaces/:workspaceId/feed").paged({
      itemIdentity: (item) => item.id,
      reconcile: signalsMod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        summaries: signalsMod.resourceValueSummaries({
          total: {
            read: (value) => value.total,
            write: (value, total) => ({ ...value, total }),
          },
        }),
      }),
      accumulatePage: (existing, next) => ({
        items: [...existing.items, ...next.items],
        cursor: next.cursor,
        total: next.total,
      }),
      load: ({ workspaceId }) => ({
        items: [{ id: `${workspaceId}:1`, title: "First" }],
        cursor: null,
        total: 1,
      }),
    });

    assert.equal(typeof feed.patch.replace, "function");
    assert.equal(typeof feed.patch.item, "function");
    assert.equal("itemAspect" in feed.patch, false);
    assert.equal("summary" in feed.patch, false);
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).paged(...) patch helpers admit page-window summaries when declared", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const feed = signals.api({}).url("/workspaces/:workspaceId/feed").paged({
      itemIdentity: (item) => item.id,
      reconcile: signalsMod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        summaries: signalsMod.resourceValueSummaries.pageWindow({
          visibleCount: {
            read: (value) => value.visibleCount,
            write: (value, visibleCount) => ({ ...value, visibleCount }),
          },
        }),
      }),
      accumulatePage: (existing, next) => ({
        items: [...existing.items, ...next.items],
        cursor: next.cursor,
        visibleCount: next.visibleCount,
      }),
      load: ({ workspaceId }) => ({
        items: [{ id: `${workspaceId}:1`, title: "First" }],
        cursor: null,
        visibleCount: 1,
      }),
    });

    assert.equal(typeof feed.patch.summary, "function");
    const line = feed.line({ workspaceId: "demo" });
    const result = line.patch(
      feed.patch.summary({
        summary: "visibleCount",
        value: 2,
      }),
    );

    assert.deepEqual(result, {
      kind: "narrowed",
      scope: "summary",
      itemId: null,
      aspect: null,
      field: null,
      summary: "visibleCount",
    });
    assert.equal(line.value().visibleCount, 2);
  } finally {
    await runtime.cleanup();
  }
});
