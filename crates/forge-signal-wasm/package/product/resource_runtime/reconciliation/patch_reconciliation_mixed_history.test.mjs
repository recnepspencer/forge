import assert from "node:assert/strict";
import test from "node:test";

import { createDeferred } from "../runtime_fixture/async/deferred.mjs";
import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";
import {
  assertLineStateUnchanged,
  captureLineState,
  normalizeForProof,
} from "./reconciliation_proof_helpers.mjs";

test("collection narrow patch then refresh converges to the same final truth as authoritative refresh", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const { mod, resource } = runtime;
    let currentValue = {
      items: [
        { id: "demo:1", title: "First" },
        { id: "demo:2", title: "Second" },
      ],
      total: 2,
    };
    const makeLine = () =>
      resource.collection({
        params: mod.resourceParams(),
        normalizeParams: ({ workspaceId }) =>
          mod.resourceParamIdentity({ workspaceId }, workspaceId),
        itemIdentity: (item) => item.id,
        reconcile: mod.resourceCollectionShape({
          items: (value) => value.items,
          replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
          aspects: mod.resourceItemAspects({
            title: {
              read: (item) => item.title,
              write: (item, value) => ({ ...item, title: String(value) }),
            },
          }),
        }),
        load: () => structuredClone(currentValue),
      }).line({ workspaceId: "demo" });

    const patchedLine = makeLine();
    patchedLine.patch(
      mod.resourcePatch.itemAspect({
        itemId: "demo:2",
        aspect: "title",
        value: "Authoritative Update",
      }),
    );
    currentValue = {
      items: [
        { id: "demo:1", title: "First" },
        { id: "demo:2", title: "Authoritative Update" },
      ],
      total: 2,
    };
    patchedLine.refresh();

    currentValue = {
      items: [
        { id: "demo:1", title: "First" },
        { id: "demo:2", title: "Second" },
      ],
      total: 2,
    };
    const refreshedLine = makeLine();
    currentValue = {
      items: [
        { id: "demo:1", title: "First" },
        { id: "demo:2", title: "Authoritative Update" },
      ],
      total: 2,
    };
    refreshedLine.refresh();

    assert.deepEqual(patchedLine.value(), refreshedLine.value());
    assert.equal(patchedLine.diagnostics().patchCount, 1);
    assert.equal(patchedLine.diagnostics().refreshCount, 1);
    assert.equal(patchedLine.history().lifecycle.at(-2)?.lastPatchScope, "aspect");
    assert.equal(patchedLine.history().lifecycle.at(-1)?.event, "fulfilled");
  } finally {
    await runtime.cleanup();
  }
});

test("patch is denied while refresh is pending without side effects", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const { mod, resource } = runtime;
    let callCount = 0;
    const refreshDeferred = createDeferred();
    const line = resource.collection({
      params: mod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        mod.resourceParamIdentity({ workspaceId }, workspaceId),
      itemIdentity: (item) => item.id,
      reconcile: mod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        aspects: mod.resourceItemAspects({
          title: {
            read: (item) => item.title,
            write: (item, value) => ({ ...item, title: String(value) }),
          },
        }),
      }),
      load: () => {
        callCount += 1;
        if (callCount === 1) {
          return { items: [{ id: "demo:1", title: "First" }] };
        }
        return refreshDeferred.promise;
      },
    }).line({ workspaceId: "demo" });

    line.refresh();
    const before = captureLineState(line);

    assert.throws(
      () =>
        line.patch(
          mod.resourcePatch.itemAspect({
            itemId: "demo:1",
            aspect: "title",
            value: "Blocked",
          }),
        ),
      /do not admit patch\(\.\.\.\) while reload is pending/,
    );

    assertLineStateUnchanged(line, before);

    refreshDeferred.resolve({ items: [{ id: "demo:1", title: "Settled" }] });
    await refreshDeferred.promise;
    await Promise.resolve();
  } finally {
    await runtime.cleanup();
  }
});

test("duplicate summary patch delivery preserves visible item truth and records summary scope twice", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const { mod, resource } = runtime;
    const line = resource.collection({
      params: mod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        mod.resourceParamIdentity({ workspaceId }, workspaceId),
      itemIdentity: (item) => item.id,
      reconcile: mod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        summaries: mod.resourceValueSummaries({
          total: {
            read: (value) => value.total,
            write: (value, total) => ({ ...value, total }),
          },
        }),
      }),
      load: () => ({
        items: [{ id: "demo:1", title: "First" }],
        total: 1,
      }),
    }).line({ workspaceId: "demo" });

    const originalItem = normalizeForProof(line.value().items[0]);
    line.patch(mod.resourcePatch.summary({ summary: "total", value: 2 }));
    const afterFirst = normalizeForProof(line.value().items[0]);
    line.patch(mod.resourcePatch.summary({ summary: "total", value: 2 }));
    const afterSecond = normalizeForProof(line.value().items[0]);

    assert.deepEqual(originalItem, afterFirst);
    assert.deepEqual(afterFirst, afterSecond);
    assert.equal(line.diagnostics().patchCount, 2);
    assert.equal(line.diagnostics().lastPatchScope, "summary");
    assert.deepEqual(
      line.history().lifecycle.slice(-2).map((entry) => ({
        event: entry.event,
        scope: entry.lastPatchScope,
        summary: entry.lastPatchedSummary,
      })),
      [
        { event: "patched", scope: "summary", summary: "total" },
        { event: "patched", scope: "summary", summary: "total" },
      ],
    );
  } finally {
    await runtime.cleanup();
  }
});

test("paged page-window summary patch preserves visible item truth through invalidation and refresh", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const { mod, resource } = runtime;
    let currentValue = {
      items: [{ id: "demo:1", title: "First" }],
      cursor: "next",
      visibleCount: 1,
    };
    const line = resource.paged({
      params: mod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        mod.resourceParamIdentity({ workspaceId }, workspaceId),
      itemIdentity: (item) => item.id,
      reconcile: mod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        summaries: mod.resourceValueSummaries.pageWindow({
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
      load: () => structuredClone(currentValue),
    }).line({ workspaceId: "demo" });

    const originalItem = normalizeForProof(line.value().items[0]);
    line.patch(mod.resourcePatch.summary({ summary: "visibleCount", value: 2 }));
    assert.deepEqual(normalizeForProof(line.value().items[0]), originalItem);
    line.invalidate();
    currentValue = {
      items: [{ id: "demo:1", title: "First" }],
      cursor: null,
      visibleCount: 3,
    };
    line.refresh();

    assert.deepEqual(line.value(), currentValue);
    assert.deepEqual(
      line.history().lifecycle.slice(-3).map((entry) => ({
        event: entry.event,
        scope: entry.lastPatchScope,
        invalidation: entry.lastInvalidationCause,
      })),
      [
        { event: "patched", scope: "summary", invalidation: null },
        { event: "invalidated", scope: "summary", invalidation: "manualLineInvalidate" },
        { event: "fulfilled", scope: "summary", invalidation: "manualLineInvalidate" },
      ],
    );
  } finally {
    await runtime.cleanup();
  }
});
