import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";
import {
  assertLineStateUnchanged,
  captureLineState,
  snapshotPatchMarker,
} from "./reconciliation_proof_helpers.mjs";

function createCollectionResource(mod, signalNamespace, load) {
  const resource = mod.createResourceNamespace(signalNamespace, {});
  return resource.collection({
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
      summaries: mod.resourceValueSummaries({
        total: {
          read: (value) => value.total,
          write: (value, total) => ({ ...value, total }),
        },
      }),
    }),
    load,
  });
}

test("narrow item, aspect, and summary patches converge to the same value truth as broad replace", async () => {
  const mod = await loadResourceModule();
  try {
    const baseValue = {
      items: [
        { id: "demo:1", title: "First" },
        { id: "demo:2", title: "Second" },
      ],
      total: 2,
    };
    const signalNamespace = createFakeSignalNamespace();

    const itemNarrow = createCollectionResource(
      mod,
      signalNamespace,
      () => structuredClone(baseValue),
    ).line({ workspaceId: "demo" });
    itemNarrow.patch(
      mod.resourcePatch.item({
        itemId: "demo:2",
        nextItem: { id: "demo:2", title: "Second Updated" },
      }),
    );

    const itemBroad = createCollectionResource(
      mod,
      signalNamespace,
      () => structuredClone(baseValue),
    ).line({ workspaceId: "demo" });
    itemBroad.patch(
      mod.resourcePatch.replace({
        items: [
          { id: "demo:1", title: "First" },
          { id: "demo:2", title: "Second Updated" },
        ],
        total: 2,
      }),
    );

    assert.deepEqual(itemNarrow.value(), itemBroad.value());
    assert.deepEqual(snapshotPatchMarker(itemNarrow), {
      diagnostics: {
        patchCount: 1,
        lastPatchKind: "item",
        lastPatchScope: "item",
        lastPatchedItemId: "demo:2",
        lastPatchedAspect: null,
        lastPatchedSummary: null,
        visibleValueVersion: 2,
      },
      historyEntry: {
        event: "patched",
        patchCount: 1,
        lastPatchKind: "item",
        lastPatchScope: "item",
        lastPatchedItemId: "demo:2",
        lastPatchedAspect: null,
        lastPatchedSummary: null,
        visibleValueVersion: 2,
      },
    });
    assert.equal(itemBroad.diagnostics().lastPatchScope, "line");
    assert.equal(itemBroad.history().lifecycle.at(-1)?.lastPatchScope, "line");

    const aspectNarrow = createCollectionResource(
      mod,
      signalNamespace,
      () => structuredClone(baseValue),
    ).line({ workspaceId: "demo" });
    aspectNarrow.patch(
      mod.resourcePatch.itemAspect({
        itemId: "demo:2",
        aspect: "title",
        value: "Aspect Updated",
      }),
    );

    const aspectBroad = createCollectionResource(
      mod,
      signalNamespace,
      () => structuredClone(baseValue),
    ).line({ workspaceId: "demo" });
    aspectBroad.patch(
      mod.resourcePatch.replace({
        items: [
          { id: "demo:1", title: "First" },
          { id: "demo:2", title: "Aspect Updated" },
        ],
        total: 2,
      }),
    );

    assert.deepEqual(aspectNarrow.value(), aspectBroad.value());
    assert.equal(aspectNarrow.diagnostics().lastPatchScope, "aspect");
    assert.equal(aspectNarrow.history().lifecycle.at(-1)?.lastPatchScope, "aspect");
    assert.equal(aspectBroad.diagnostics().lastPatchScope, "line");
    assert.equal(aspectBroad.history().lifecycle.at(-1)?.lastPatchScope, "line");

    const summaryNarrow = createCollectionResource(
      mod,
      signalNamespace,
      () => structuredClone(baseValue),
    ).line({ workspaceId: "demo" });
    summaryNarrow.patch(
      mod.resourcePatch.summary({
        summary: "total",
        value: 3,
      }),
    );

    const summaryBroad = createCollectionResource(
      mod,
      signalNamespace,
      () => structuredClone(baseValue),
    ).line({ workspaceId: "demo" });
    summaryBroad.patch(
      mod.resourcePatch.replace({
        items: [
          { id: "demo:1", title: "First" },
          { id: "demo:2", title: "Second" },
        ],
        total: 3,
      }),
    );

    assert.deepEqual(summaryNarrow.value(), summaryBroad.value());
    assert.equal(summaryNarrow.diagnostics().lastPatchScope, "summary");
    assert.equal(summaryNarrow.history().lifecycle.at(-1)?.lastPatchScope, "summary");
    assert.equal(summaryBroad.diagnostics().lastPatchScope, "line");
    assert.equal(summaryBroad.history().lifecycle.at(-1)?.lastPatchScope, "line");
  } finally {
    await mod.cleanup();
  }
});

test("paged narrow patch denies off-page items without side effects", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const feed = resource.paged({
      params: mod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        mod.resourceParamIdentity({ workspaceId }, workspaceId),
      itemIdentity: (item) => item.id,
      reconcile: mod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      }),
      accumulatePage: (existing, next) => ({
        items: [...existing.items, ...next.items],
        cursor: next.cursor,
      }),
      load: ({ workspaceId }) => ({
        items: [{ id: `${workspaceId}:visible`, title: "Visible" }],
        cursor: "next-page",
      }),
    });

    const line = feed.line({ workspaceId: "demo" });
    const before = captureLineState(line);

    assert.throws(
      () =>
        line.patch(
          mod.resourcePatch.item({
            itemId: "demo:off-page",
            nextItem: { id: "demo:off-page", title: "Nope" },
          }),
        ),
      /could not find itemId "demo:off-page"/,
    );

    assertLineStateUnchanged(line, before);
  } finally {
    await mod.cleanup();
  }
});

test("paged narrow patch denies duplicated logical items inside one visible accumulated window", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const feed = resource.paged({
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
      accumulatePage: (existing, next) => ({
        items: [...existing.items, ...next.items],
        cursor: next.cursor,
      }),
      load: ({ workspaceId }) => ({
        items: [
          { id: `${workspaceId}:shared`, title: "Page One Copy" },
          { id: `${workspaceId}:shared`, title: "Page Two Copy" },
        ],
        cursor: "page-3",
      }),
    });

    const line = feed.line({ workspaceId: "demo" });
    const before = captureLineState(line);

    assert.throws(
      () =>
        line.patch(
          mod.resourcePatch.itemAspect({
            itemId: "demo:shared",
            aspect: "title",
            value: "Should Deny",
          }),
        ),
      /duplicated visible itemId "demo:shared"/,
    );

    assertLineStateUnchanged(line, before);
  } finally {
    await mod.cleanup();
  }
});

test("narrow item patch denies duplicated visible identities without side effects", async () => {
  const mod = await loadResourceModule();
  try {
    const line = createCollectionResource(
      mod,
      createFakeSignalNamespace(),
      () => ({
        items: [
          { id: "demo:dup", title: "First Copy" },
          { id: "demo:dup", title: "Second Copy" },
        ],
        total: 2,
      }),
    ).line({ workspaceId: "demo" });

    const before = captureLineState(line);

    assert.throws(
      () =>
        line.patch(
          mod.resourcePatch.item({
            itemId: "demo:dup",
            nextItem: { id: "demo:dup", title: "Updated" },
          }),
        ),
      /duplicated visible itemId "demo:dup"/,
    );

    assertLineStateUnchanged(line, before);
  } finally {
    await mod.cleanup();
  }
});

test("narrow item patch denies identity-changing replacement without side effects", async () => {
  const mod = await loadResourceModule();
  try {
    const line = createCollectionResource(
      mod,
      createFakeSignalNamespace(),
      () => ({
        items: [{ id: "demo:1", title: "First" }],
        total: 1,
      }),
    ).line({ workspaceId: "demo" });

    const before = captureLineState(line);

    assert.throws(
      () =>
        line.patch(
          mod.resourcePatch.item({
            itemId: "demo:1",
            nextItem: { id: "demo:2", title: "Wrong Identity" },
          }),
        ),
      /preserve item identity "demo:1"/,
    );

    assertLineStateUnchanged(line, before);
  } finally {
    await mod.cleanup();
  }
});
