import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";

test("collection lines narrow item-aspect patches without broad reload", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const { mod, resource } = runtime;
    let loadCount = 0;
    const products = resource.collection({
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
        aspects: mod.resourceItemAspects({
          title: {
            read: (item) => item.title,
            write: (item, value) => ({ ...item, title: String(value) }),
          },
        }),
      }),
      load: ({ workspaceId }) => {
        loadCount += 1;
        return {
          items: [
            { id: `${workspaceId}:1`, title: "First" },
            { id: `${workspaceId}:2`, title: "Second" },
          ],
          total: 2,
        };
      },
    });

    const line = products.line({ workspaceId: "demo" });
    assert.deepEqual(line.reconciliation(), {
      broadReplace: true,
      narrowItem: true,
      narrowSummary: true,
      aspectNames: ["title"],
      summaryNames: ["total"],
    });
    const result = line.patch(
      mod.resourcePatch.itemAspect({
        itemId: "demo:2",
        aspect: "title",
        value: "Updated Second",
      }),
    );

    assert.deepEqual(result, {
      kind: "narrowed",
      scope: "aspect",
      itemId: "demo:2",
      aspect: "title",
    });
    assert.equal(loadCount, 1);
    assert.deepEqual(line.value(), {
      items: [
        { id: "demo:1", title: "First" },
        { id: "demo:2", title: "Updated Second" },
      ],
      total: 2,
    });
    assert.equal(line.diagnostics().patchCount, 1);
    assert.equal(line.diagnostics().lastPatchKind, "itemAspect");
    assert.equal(line.diagnostics().lastPatchScope, "aspect");
    assert.equal(line.diagnostics().lastPatchedItemId, "demo:2");
    assert.equal(line.diagnostics().lastPatchedAspect, "title");
    assert.equal(line.diagnostics().lastPatchedSummary, null);
    assert.equal(line.history().lifecycle.at(-1)?.event, "patched");
  } finally {
    await runtime.cleanup();
  }
});

test("collection lines admit broad replace patch without declared reconcile", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const { mod, resource } = runtime;
    let loadCount = 0;
    const products = resource.collection({
      params: mod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        mod.resourceParamIdentity({ workspaceId }, workspaceId),
      itemIdentity: (item) => item.id,
      load: ({ workspaceId }) => {
        loadCount += 1;
        return [{ id: `${workspaceId}:1`, title: "First" }];
      },
    });

    const line = products.line({ workspaceId: "demo" });
    assert.deepEqual(line.reconciliation(), {
      broadReplace: true,
      narrowItem: false,
      narrowSummary: false,
      aspectNames: [],
      summaryNames: [],
    });
    const replaceResult = line.patch(
      mod.resourcePatch.replace([{ id: "demo:1", title: "Replaced" }]),
    );

    assert.deepEqual(replaceResult, {
      kind: "replaced",
      scope: "line",
      itemId: null,
      aspect: null,
    });
    assert.equal(loadCount, 1);
    assert.deepEqual(line.value(), [{ id: "demo:1", title: "Replaced" }]);
    assert.throws(
      () =>
        line.patch(
          mod.resourcePatch.item({
            itemId: "demo:1",
            nextItem: { id: "demo:1", title: "Nope" },
          }),
        ),
      /require reconcile: resourceCollectionShape/,
    );
    assert.throws(
      () =>
        line.patch({
          kind: "item",
          itemId: "demo:1",
          nextItem: { id: "demo:1", title: "Nope" },
        }),
      /resourcePatch\.\*\(\)/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("paged lines admit narrow item patch when reconciliation is declared", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const { mod, resource } = runtime;
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
        items: [{ id: `${workspaceId}:1`, title: "First" }],
        cursor: null,
      }),
    });

    const line = feed.line({ workspaceId: "demo" });
    assert.deepEqual(line.reconciliation(), {
      broadReplace: true,
      narrowItem: true,
      narrowSummary: false,
      aspectNames: [],
      summaryNames: [],
    });
    const result = line.patch(
      mod.resourcePatch.item({
        itemId: "demo:1",
        nextItem: { id: "demo:1", title: "Paged Updated" },
      }),
    );

    assert.deepEqual(result, {
      kind: "narrowed",
      scope: "item",
      itemId: "demo:1",
      aspect: null,
    });
    assert.deepEqual(line.value(), {
      items: [{ id: "demo:1", title: "Paged Updated" }],
      cursor: null,
    });
    assert.equal(line.diagnostics().lastPatchScope, "item");
  } finally {
    await runtime.cleanup();
  }
});

test("collection lines admit summary patch when summaries are declared", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const { mod, resource } = runtime;
    let loadCount = 0;
    const products = resource.collection({
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
      load: ({ workspaceId }) => {
        loadCount += 1;
        return {
          items: [{ id: `${workspaceId}:1`, title: "First" }],
          total: 1,
        };
      },
    });

    const line = products.line({ workspaceId: "demo" });
    const result = line.patch(
      mod.resourcePatch.summary({
        summary: "total",
        value: 2,
      }),
    );

    assert.deepEqual(result, {
      kind: "narrowed",
      scope: "summary",
      itemId: null,
      aspect: null,
      summary: "total",
    });
    assert.equal(loadCount, 1);
    assert.deepEqual(line.value(), {
      items: [{ id: "demo:1", title: "First" }],
      total: 2,
    });
    assert.equal(line.diagnostics().lastPatchKind, "summary");
    assert.equal(line.diagnostics().lastPatchScope, "summary");
    assert.equal(line.diagnostics().lastPatchedSummary, "total");
    assert.equal(line.history().lifecycle.at(-1)?.lastPatchedSummary, "total");
  } finally {
    await runtime.cleanup();
  }
});

test("summary patch is denied when summary write mutates reconciled items", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const { mod, resource } = runtime;
    const products = resource.collection({
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
            write: (value, total) => ({
              items: [{ ...value.items[0], title: `total:${total}` }],
              total,
            }),
          },
        }),
      }),
      load: ({ workspaceId }) => ({
        items: [{ id: `${workspaceId}:1`, title: "First" }],
        total: 1,
      }),
    });

    const line = products.line({ workspaceId: "demo" });

    assert.throws(
      () =>
        line.patch(
          mod.resourcePatch.summary({
            summary: "total",
            value: 2,
          }),
        ),
      /preserve item objects/,
    );
    assert.deepEqual(line.value(), {
      items: [{ id: "demo:1", title: "First" }],
      total: 1,
    });
    assert.equal(line.diagnostics().patchCount, 0);
    assert.equal(line.history().lifecycle.at(-1)?.event, "materialized");
  } finally {
    await runtime.cleanup();
  }
});

test("detail lines remain non-patchable while collection lines deny undeclared aspects", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const { mod, resource } = runtime;
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => ({ id: productId }),
    });
    const collection = resource.collection({
      params: mod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        mod.resourceParamIdentity({ workspaceId }, workspaceId),
      itemIdentity: (item) => item.id,
      reconcile: mod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      }),
      load: ({ workspaceId }) => ({
        items: [{ id: `${workspaceId}:1`, title: "First" }],
      }),
    });

    const detailLine = detail.line({ productId: "p1" });
    const collectionLine = collection.line({ workspaceId: "demo" });

    assert.equal("patch" in detailLine, false);
    assert.throws(
      () =>
        collectionLine.patch(
          mod.resourcePatch.itemAspect({
            itemId: "demo:1",
            aspect: "title",
            value: "No Aspect Declaration",
          }),
        ),
      /undeclared aspect "title"/,
    );
    assert.throws(
      () =>
        collectionLine.patch(
          mod.resourcePatch.summary({
            summary: "total",
            value: 1,
          }),
        ),
      /undeclared summary "total"/,
    );
  } finally {
    await runtime.cleanup();
  }
});
