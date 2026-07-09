import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";

test("api.url(...).list(...) families own native delivery helpers without creating a second delivery engine", async () => {
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

    assert.equal(typeof catalog.delivery.replace, "function");
    assert.equal(typeof catalog.delivery.patch, "function");
    assert.equal(typeof catalog.delivery.invalidate, "function");
    assert.equal(typeof catalog.delivery.item, "function");
    assert.equal(typeof catalog.delivery.itemAspect, "function");
    assert.equal(typeof catalog.delivery.summary, "function");
    assert.deepEqual(
      catalog.delivery.replace({
        packetId: "pkt-replace",
        basisId: null,
        nextBasisId: "basis-1",
        nextValue: {
          items: [{ id: "demo:1", title: "Replaced" }],
          total: 2,
        },
      }),
      signalsMod.resourceDelivery.replace({
        packetId: "pkt-replace",
        basisId: null,
        nextBasisId: "basis-1",
        nextValue: {
          items: [{ id: "demo:1", title: "Replaced" }],
          total: 2,
        },
      }),
    );
    assert.deepEqual(
      catalog.delivery.invalidate({
        packetId: "pkt-invalidate",
        basisId: null,
        nextBasisId: "basis-1",
      }),
      signalsMod.resourceDelivery.invalidate({
        packetId: "pkt-invalidate",
        basisId: null,
        nextBasisId: "basis-1",
      }),
    );
    assert.deepEqual(
      catalog.delivery.itemAspect({
        packetId: "pkt-1",
        basisId: null,
        nextBasisId: "basis-1",
        itemId: "demo:1",
        aspect: "title",
        value: "Delivered",
      }),
      signalsMod.resourceDelivery.patch({
        packetId: "pkt-1",
        basisId: null,
        nextBasisId: "basis-1",
        patch: signalsMod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "Delivered",
        }),
      }),
    );

    const line = catalog.line({ workspaceId: "demo" });
    const deliveryResult = line.deliver(
      catalog.delivery.summary({
        packetId: "pkt-summary",
        basisId: null,
        nextBasisId: "basis-1",
        summary: "total",
        value: 2,
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
    assert.deepEqual(line.value(), {
      items: [{ id: "demo:1", title: "First" }],
      total: 2,
    });
    assert.equal(line.diagnostics().lastDeliveryScope, "summary");
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).paged(...) delivery helpers do not overclaim line-scoped summary delivery admission", async () => {
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

    assert.equal(typeof feed.delivery.replace, "function");
    assert.equal(typeof feed.delivery.patch, "function");
    assert.equal(typeof feed.delivery.invalidate, "function");
    assert.equal(typeof feed.delivery.item, "function");
    assert.deepEqual(
      feed.delivery.replace({
        packetId: "pkt-replace",
        basisId: null,
        nextBasisId: "basis-1",
        nextValue: {
          items: [{ id: "demo:replaced", title: "Replaced" }],
          cursor: "cursor-1",
          total: 2,
        },
      }),
      signalsMod.resourceDelivery.replace({
        packetId: "pkt-replace",
        basisId: null,
        nextBasisId: "basis-1",
        nextValue: {
          items: [{ id: "demo:replaced", title: "Replaced" }],
          cursor: "cursor-1",
          total: 2,
        },
      }),
    );
    assert.deepEqual(
      feed.delivery.invalidate({
        packetId: "pkt-invalidate",
        basisId: null,
        nextBasisId: "basis-1",
      }),
      signalsMod.resourceDelivery.invalidate({
        packetId: "pkt-invalidate",
        basisId: null,
        nextBasisId: "basis-1",
      }),
    );
    assert.equal("itemAspect" in feed.delivery, false);
    assert.equal("summary" in feed.delivery, false);
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).paged(...) delivery helpers admit page-window summary delivery when declared", async () => {
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

    assert.equal(typeof feed.delivery.summary, "function");
    const line = feed.line({ workspaceId: "demo" });
    const deliveryResult = line.deliver(
      feed.delivery.summary({
        packetId: "pkt-visible-count",
        basisId: null,
        nextBasisId: "basis-1",
        summary: "visibleCount",
        value: 2,
      }),
    );

    assert.deepEqual(deliveryResult, {
      kind: "applied",
      deliveryKind: "patch",
      scope: "summary",
      packetId: "pkt-visible-count",
      basisId: null,
      nextBasisId: "basis-1",
      supersededOperation: null,
    });
    assert.equal(line.value().visibleCount, 2);
  } finally {
    await runtime.cleanup();
  }
});
