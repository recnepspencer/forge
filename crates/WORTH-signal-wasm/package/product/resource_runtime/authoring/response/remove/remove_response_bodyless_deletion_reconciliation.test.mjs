import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("remove responses can reconcile exact deletion from metadata-only summary truth when collection.itemId is declared", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskList = runtime.signals.api({}).url("/task-search")
      .response(runtime.signals.resource.response.collection({
        itemId: (item) => item.id,
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        summaries: runtime.signalsMod.resourceValueSummaries({
          total: {
            read: (value) => value.total,
            write: (value, total) => ({ ...value, total }),
          },
        }),
      }))
      .list({
        load: () => ({
          items: [
            { id: "t1", title: "First" },
            { id: "t2", title: "Second" },
          ],
          total: 2,
        }),
      });
    const line = taskList.line({});

    const plan = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.summary()())
      .remove({
        reconciles: [
          {
            family: taskList,
            params: () => ({}),
            fallback: "deletionUnavailable",
            collection: {
              kind: "delete",
              itemId: ({ taskId }) => taskId,
            },
          },
          {
            family: taskList,
            params: () => ({}),
            fallback: "refetchRequired",
            summary: { kind: "summary", summary: "total" },
          },
        ],
        load: () => 1,
      })
      .line({ taskId: "t1" })
      .mutationResponse();

    assert.deepEqual(line.value(), {
      items: [{ id: "t2", title: "Second" }],
      total: 1,
    });
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionDelete");
    assert.equal(plan.executionArtifacts[0].itemId, "t1");
    assert.equal(plan.targets[0].reconciliation.targetDigest, "collection:delete:declaredItemId");
    assert.equal(plan.executionArtifacts[1].kind, "exactSummary");
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.equal(plan.confirmation.exactTargetCount, 2);
  } finally {
    await runtime.cleanup();
  }
});

test("remove responses can reconcile exact deletion from bodyless truth when collection.itemId is declared", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskList = runtime.signals.api({}).url("/task-search")
      .response(runtime.signals.resource.response.array({
        itemId: (item) => item.id,
      }))
      .list({
        load: () => [
          { id: "t1", title: "First" },
          { id: "t2", title: "Second" },
        ],
      });
    const line = taskList.line({});

    const plan = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.summary()())
      .remove({
        reconciles: [
          {
            family: taskList,
            params: () => ({}),
            fallback: "deletionUnavailable",
            collection: {
              kind: "delete",
              itemId: ({ taskId }) => taskId,
            },
          },
        ],
        load: () => undefined,
      })
      .line({ taskId: "t1" })
      .mutationResponse();

    assert.deepEqual(line.value(), [{ id: "t2", title: "Second" }]);
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionDelete");
    assert.equal(plan.executionArtifacts[0].itemId, "t1");
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.equal(plan.confirmation.exactTargetCount, 1);
  } finally {
    await runtime.cleanup();
  }
});

test("metadata-only remove deletion is denied without an explicit collection.itemId declaration", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskList = runtime.signals.api({}).url("/task-search")
      .response(runtime.signals.resource.response.array({
        itemId: (item) => item.id,
      }))
      .list({
        load: () => [{ id: "t1", title: "First" }],
      });

    assert.throws(
      () => runtime.signals.api({}).url("/tasks/:taskId")
        .response(runtime.signals.resource.response.summary()())
        .remove({
          reconciles: [
            {
              family: taskList,
              params: () => ({}),
              fallback: "deletionUnavailable",
              collection: { kind: "delete" },
            },
          ],
          load: () => 0,
        }),
      /collection delete reconciliation requires collection\.itemId\(\.\.\.\) when the mutation response lens does not carry canonical item identity/,
    );
  } finally {
    await runtime.cleanup();
  }
});
