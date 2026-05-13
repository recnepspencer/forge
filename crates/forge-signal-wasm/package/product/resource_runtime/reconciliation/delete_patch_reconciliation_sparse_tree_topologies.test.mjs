import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";
import { createTreeTasks } from "../runtime_fixture/tree_collection_runtime_fixture.mjs";
import {
  assertLineStateUnchanged,
  captureLineState,
} from "./reconciliation_proof_helpers.mjs";

test("sparse-page route families admit exact delete patch through loaded-page reconstruction", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const sparseTasks = runtime.signals.api({}).url("/sparse-tasks")
      .response(runtime.signals.resource.response.sparse()({
        itemId: (task) => task.id,
        pageId: (task) => task.page,
        pageForItem: () => "page-1",
        pages: (value) => value.pages,
        replacePages: (value, nextPages) => ({ ...value, pages: nextPages }),
        replacePageItem: (value, pageId, itemId, nextItem) => ({
          ...value,
          pages: Object.fromEntries(
            Object.entries(value.pages).map(([key, items]) => [
              key,
              key === pageId
                ? items.map((item) => item.id === itemId ? nextItem : item)
                : items,
            ]),
          ),
        }),
      }))
      .list({
        load: () => ({
          pages: {
            "page-1": [
              { id: "task:1", page: "page-1", title: "First" },
              { id: "task:2", page: "page-1", title: "Second" },
            ],
            "page-2": [{ id: "task:3", page: "page-2", title: "Third" }],
          },
        }),
      });
    const line = sparseTasks.line({});

    line.patch(sparseTasks.patch.delete({ itemId: "task:1" }));

    assert.deepEqual(line.value(), {
      pages: {
        "page-1": [{ id: "task:2", page: "page-1", title: "Second" }],
        "page-2": [{ id: "task:3", page: "page-2", title: "Third" }],
      },
    });
    assert.equal(line.diagnostics().lastPatchKind, "delete");
    assert.equal(line.diagnostics().lastPatchedItemId, "task:1");
    assert.deepEqual(line.diagnostics().lastEffect.locusProof.cost, {
      lookup: "sparse-page-item-id",
      lookupBreadth: 1,
      traversal: "loaded-page-chunk",
      traversalBreadth: 1,
      reconstruction: "replacePages",
      reconstructionBreadth: 1,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("sparse-page delete patch denies page lookup mismatch without side effects", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const sparseTasks = runtime.signals.api({}).url("/sparse-tasks")
      .response(runtime.signals.resource.response.sparse()({
        itemId: (task) => task.id,
        pageId: (task) => task.page,
        pageForItem: (itemId) => itemId === "task:1" ? "page-2" : "page-1",
        pages: (value) => value.pages,
        replacePages: (value, nextPages) => ({ ...value, pages: nextPages }),
        replacePageItem: (value, pageId, itemId, nextItem) => ({
          ...value,
          pages: Object.fromEntries(
            Object.entries(value.pages).map(([key, items]) => [
              key,
              key === pageId
                ? items.map((item) => item.id === itemId ? nextItem : item)
                : items,
            ]),
          ),
        }),
      }))
      .list({
        load: () => ({
          pages: {
            "page-1": [{ id: "task:1", page: "page-1", title: "First" }],
            "page-2": [],
          },
        }),
      });
    const line = sparseTasks.line({});
    const before = captureLineState(line);

    assert.throws(
      () => line.patch(sparseTasks.patch.delete({ itemId: "task:1" })),
      /sparse page lookup page id "page-2" to match actual item page id "page-1"/,
    );

    assertLineStateUnchanged(line, before);
  } finally {
    await runtime.cleanup();
  }
});

test("tree route families admit exact delete patch through declared descendant-path reconstruction", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const treeTasks = createTreeTasks(runtime, "/tree-tasks", {
      nodeForItem: (itemId) => itemId === "root" ? ["root"] : ["root", itemId],
    });
    const line = treeTasks.line({});

    line.patch(treeTasks.patch.delete({ itemId: "task:1" }));

    assert.deepEqual(line.value(), {
      roots: [{
        id: "root",
        title: "Root",
        children: [],
      }],
    });
    assert.equal(line.diagnostics().lastPatchKind, "delete");
    assert.equal(line.diagnostics().lastPatchedItemId, "task:1");
    assert.deepEqual(line.diagnostics().lastEffect.locusProof.cost, {
      lookup: "tree-descendant-path",
      lookupBreadth: 1,
      traversal: "single-descendant-path",
      traversalBreadth: 1,
      reconstruction: "replaceChildrenOrRoots",
      reconstructionBreadth: 1,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("tree delete patch denies lookup path mismatch without side effects", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const treeTasks = createTreeTasks(runtime, "/tree-tasks", {
      nodeForItem: (itemId) => itemId === "root" ? ["root"] : ["missing-parent", itemId],
    });
    const line = treeTasks.line({});
    const before = captureLineState(line);

    assert.throws(
      () => line.patch(treeTasks.patch.delete({ itemId: "task:1" })),
      /tree lookup path "missing-parent > task:1" to match actual node path "root > task:1"/,
    );

    assertLineStateUnchanged(line, before);
  } finally {
    await runtime.cleanup();
  }
});
