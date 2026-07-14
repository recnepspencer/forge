import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";
import { createTreeTasks } from "../../../runtime_fixture/tree_collection_runtime_fixture.mjs";

test("remove responses can delete sparse-page items through exact loaded-page reconstruction", async () => {
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
    const sparseLine = sparseTasks.line({});

    const plan = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .remove({
        reconciles: [{
          family: sparseTasks,
          params: () => ({}),
          fallback: "deletionUnavailable",
          collection: { kind: "delete" },
        }],
        load: ({ taskId }) => ({ id: taskId }),
      })
      .line({ taskId: "task:1" })
      .mutationResponse();

    assert.deepEqual(sparseLine.value(), {
      pages: {
        "page-1": [{ id: "task:2", page: "page-1", title: "Second" }],
        "page-2": [{ id: "task:3", page: "page-2", title: "Third" }],
      },
    });
    assert.equal(plan.targets[0].line.residency, "resident");
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionDelete");
    assert.deepEqual(sparseLine.diagnostics().lastEffect.locusProof.cost, {
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

test("remove responses can delete tree items through exact descendant-path reconstruction", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const treeTasks = createTreeTasks(runtime, "/tree-tasks", {
      nodeForItem: (itemId) => itemId === "root" ? ["root"] : ["root", itemId],
    });
    const treeLine = treeTasks.line({});

    const plan = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .remove({
        reconciles: [{
          family: treeTasks,
          params: () => ({}),
          fallback: "deletionUnavailable",
          collection: { kind: "delete" },
        }],
        load: ({ taskId }) => ({ id: taskId }),
      })
      .line({ taskId: "task:1" })
      .mutationResponse();

    assert.deepEqual(treeLine.value(), {
      roots: [{
        id: "root",
        title: "Root",
        children: [],
      }],
    });
    assert.equal(plan.targets[0].line.residency, "resident");
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionDelete");
    assert.deepEqual(treeLine.diagnostics().lastEffect.locusProof.cost, {
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
