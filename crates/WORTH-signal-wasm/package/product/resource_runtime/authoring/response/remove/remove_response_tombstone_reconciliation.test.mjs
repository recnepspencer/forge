import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("remove responses can retain a collection item as a canonical tombstone", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskList = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.collection({
        itemId: (item) => item.id,
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      }))
      .list({
        load: () => ({
          items: [
            { id: "t1", title: "First", status: "active" },
            { id: "t2", title: "Second", status: "active" },
          ],
        }),
      });
    const taskLine = taskList.line({});

    const plan = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .remove({
        reconciles: [{
          family: taskList,
          params: () => ({}),
          fallback: "refetchRequired",
          collection: { kind: "item" },
        }],
        load: ({ taskId }) => ({
          id: taskId,
          title: "First",
          status: "deleted",
        }),
      })
      .line({ taskId: "t1" })
      .mutationResponse();

    assert.deepEqual(taskLine.value(), {
      items: [
        { id: "t1", title: "First", status: "deleted" },
        { id: "t2", title: "Second", status: "active" },
      ],
    });
    assert.equal(plan.method, "DELETE");
    assert.equal(plan.targets[0].reconciliation.kind, "item");
    assert.equal(plan.targets[0].reconciliation.targetDigest, "collection:tombstone");
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionTombstone");
    assert.equal(plan.executionArtifacts[0].scope, "item");
    assert.equal(plan.executionArtifacts[0].itemId, "t1");
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.equal(plan.counters.appliedTargetBreadth, 1);
    assert.equal(taskLine.diagnostics().lastDeliveryScope, "item");
    assert.equal(taskLine.diagnostics().lastPatchedItemId, "t1");
    assert.equal(taskLine.history().lifecycle.at(-1)?.event, "delivered");
  } finally {
    await runtime.cleanup();
  }
});

test("remove responses can retain a paged item as a tombstone through canonical item replacement", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const feed = runtime.signals.api({}).url("/feed").paged({
      itemIdentity: (item) => item.id,
      reconcile: runtime.signalsMod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      }),
      accumulatePage: (existing, next) => ({
        items: [...existing.items, ...next.items],
        cursor: next.cursor,
      }),
      load: () => ({
        items: [
          { id: "t1", title: "First", status: "active" },
          { id: "t2", title: "Second", status: "active" },
        ],
        cursor: "next",
      }),
    });
    const feedLine = feed.line({});

    const plan = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .remove({
        reconciles: [
          {
            family: feed,
            params: () => ({}),
            fallback: "refetchRequired",
            collection: { kind: "item" },
          },
        ],
        load: ({ taskId }) => ({
          id: taskId,
          title: "First",
          status: "deleted",
        }),
      })
      .line({ taskId: "t1" })
      .mutationResponse();

    assert.deepEqual(feedLine.value(), {
      items: [
        { id: "t1", title: "First", status: "deleted" },
        { id: "t2", title: "Second", status: "active" },
      ],
      cursor: "next",
    });
    assert.equal(plan.method, "DELETE");
    assert.equal(plan.targets[0].reconciliation.kind, "item");
    assert.equal(plan.targets[0].reconciliation.targetDigest, "collection:tombstone");
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionTombstone");
    assert.equal(plan.executionArtifacts[0].familyKind, "paged");
    assert.equal(feedLine.diagnostics().lastDeliveryScope, "item");
    assert.equal(feedLine.diagnostics().lastPatchedItemId, "t1");
  } finally {
    await runtime.cleanup();
  }
});
