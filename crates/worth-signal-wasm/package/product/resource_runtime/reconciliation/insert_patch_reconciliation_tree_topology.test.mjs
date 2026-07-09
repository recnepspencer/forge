import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";
import { createTreeTasks } from "../runtime_fixture/tree_collection_runtime_fixture.mjs";
import {
  assertLineStateUnchanged,
  captureLineState,
} from "./reconciliation_proof_helpers.mjs";

test("tree route families admit exact insert patch through declared parent-path reconstruction", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const treeTasks = createTreeTasks(runtime, "/tree-tasks", {
      nodeForItem: (itemId) => itemId === "root" ? ["root"] : ["root", itemId],
    });
    const line = treeTasks.line({});

    line.patch(treeTasks.patch.insert({
      itemId: "task:2",
      placement: "append",
      nextItem: { id: "task:2", title: "Second", children: [] },
    }));

    assert.deepEqual(line.value(), {
      roots: [{
        id: "root",
        title: "Root",
        children: [
          { id: "task:1", title: "First", children: [] },
          { id: "task:2", title: "Second", children: [] },
        ],
      }],
    });
    assert.equal(line.diagnostics().lastPatchKind, "insert");
    assert.equal(line.diagnostics().lastPatchedItemId, "task:2");
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

test("tree route families deny insert when the declared parent path does not exist without side effects", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const treeTasks = createTreeTasks(runtime, "/tree-tasks", {
      nodeForItem: (itemId) => ["missing-parent", itemId],
    });
    const line = treeTasks.line({});
    const before = captureLineState(line);

    assert.throws(
      () =>
        line.patch(treeTasks.patch.insert({
          itemId: "task:2",
          placement: "append",
          nextItem: { id: "task:2", title: "Second", children: [] },
        })),
      /tree parent path "missing-parent" to resolve an existing parent node/,
    );

    assertLineStateUnchanged(line, before);
  } finally {
    await runtime.cleanup();
  }
});
