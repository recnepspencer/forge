import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("save responses can replace resident collection items through canonical mutation reconciliation", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const collectionResponse = runtime.signals.resource.response.array({
      itemId: (item) => item.id,
    });
    const taskList = runtime.signals.api({}).url("/tasks").response(collectionResponse).list({
      load: () => [{ id: "t1", title: "First" }],
    });
    const taskLine = taskList.line({});
    const saveTask = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .update({
        reconciles: [
          {
            family: taskList,
            params: () => ({}),
            fallback: "refetchRequired",
            collection: { kind: "item" },
          },
        ],
        load: ({ taskId, body }) => ({ id: taskId, title: body.title }),
      });

    const saveLine = saveTask.line({
      taskId: "t1",
      body: { title: "Updated" },
    });
    const plan = saveLine.mutationResponse();

    assert.deepEqual(taskLine.value(), [{ id: "t1", title: "Updated" }]);
    assert.equal(plan.targets[0].reconciliation.kind, "item");
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionItem");
    assert.equal(plan.executionArtifacts[0].scope, "item");
    assert.equal(plan.executionArtifacts[0].itemId, "t1");
    assert.equal(plan.executionArtifacts[0].deliveryKind, "patch");
    assert.equal(plan.executionArtifacts[0].deliveryScope, "item");
    assert.equal(taskLine.diagnostics().lastDeliveryKind, "patch");
    assert.equal(taskLine.diagnostics().lastDeliveryScope, "item");
    assert.equal(taskLine.diagnostics().lastPatchedItemId, "t1");
    assert.equal(taskLine.diagnostics().lastEffect.provenance, "deliveredPatch");
    assert.equal(plan.counters.fallbackBreadth, 0);
    assert.equal(plan.counters.appliedTargetBreadth, 1);
  } finally {
    await runtime.cleanup();
  }
});

test("save responses can patch collection summaries from declared response fields", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const collectionResponse = runtime.signals.resource.response.collection({
      itemId: (item) => item.id,
      items: (value) => value.items,
      replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      summaries: runtime.signalsMod.resourceValueSummaries({
        total: {
          read: (value) => value.total,
          write: (value, total) => ({ ...value, total }),
        },
      }),
    });
    const taskList = runtime.signals.api({}).url("/task-search").response(collectionResponse).list({
      load: () => ({
        items: [{ id: "t1", title: "First" }],
        total: 1,
      }),
    });
    const taskLine = taskList.line({});
    const saveStats = runtime.signals.api({}).url("/task-search/stats")
      .response(runtime.signals.resource.response.detail()({ total: "total" }))
      .update({
        reconciles: [
          {
            family: taskList,
            params: () => ({}),
            fallback: "refetchRequired",
            summary: { kind: "summary", summary: "total" },
          },
        ],
        load: ({ body }) => ({ total: body.total }),
      });

    const saveLine = saveStats.line({ body: { total: 2 } });
    const plan = saveLine.mutationResponse();

    assert.deepEqual(taskLine.value(), {
      items: [{ id: "t1", title: "First" }],
      total: 2,
    });
    assert.equal(plan.targets[0].reconciliation.kind, "summary");
    assert.equal(plan.targets[0].reconciliation.summary, "total");
    assert.equal(plan.executionArtifacts[0].kind, "exactSummary");
    assert.equal(plan.executionArtifacts[0].scope, "summary");
    assert.equal(plan.executionArtifacts[0].summary, "total");
    assert.equal(plan.executionArtifacts[0].deliveryKind, "patch");
    assert.equal(plan.executionArtifacts[0].deliveryScope, "summary");
    assert.equal(taskLine.diagnostics().lastDeliveryScope, "summary");
    assert.equal(taskLine.diagnostics().lastPatchedSummary, "total");
    assert.equal(plan.counters.appliedTargetBreadth, 1);
  } finally {
    await runtime.cleanup();
  }
});

test("save responses can replace resident paged items through canonical mutation reconciliation", async () => {
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
        items: [{ id: "t1", title: "First" }],
        cursor: "next",
      }),
    });
    const feedLine = feed.line({});
    const saveTask = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .update({
        reconciles: [
          {
            family: feed,
            params: () => ({}),
            fallback: "refetchRequired",
            collection: { kind: "item" },
          },
        ],
        load: ({ taskId, body }) => ({ id: taskId, title: body.title }),
      });

    const plan = saveTask.line({
      taskId: "t1",
      body: { title: "Paged Updated" },
    }).mutationResponse();

    assert.deepEqual(feedLine.value(), {
      items: [{ id: "t1", title: "Paged Updated" }],
      cursor: "next",
    });
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionItem");
    assert.equal(plan.executionArtifacts[0].familyKind, "paged");
    assert.equal(plan.executionArtifacts[0].scope, "item");
    assert.equal(plan.executionArtifacts[0].itemId, "t1");
    assert.equal(plan.executionArtifacts[0].submittedTarget.familyKind, "paged");
    assert.equal(plan.counters.appliedTargetBreadth, 1);
    assert.equal(plan.counters.targetBasisSnapshotBreadth, 1);
    assert.equal(feedLine.diagnostics().lastDeliveryScope, "item");
    assert.equal(feedLine.diagnostics().lastPatchedItemId, "t1");
  } finally {
    await runtime.cleanup();
  }
});

test("save responses patch paged page-window summaries from declared response fields", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const feed = runtime.signals.api({}).url("/feed-counts").paged({
      itemIdentity: (item) => item.id,
      reconcile: runtime.signalsMod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        summaries: runtime.signalsMod.resourceValueSummaries.pageWindow({
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
      load: () => ({
        items: [{ id: "t1", title: "First" }],
        cursor: "next",
        visibleCount: 1,
      }),
    });
    const feedLine = feed.line({});
    const saveStats = runtime.signals.api({}).url("/feed-counts/stats")
      .response(runtime.signals.resource.response.detail()({
        visibleCount: "visibleCount",
      }))
      .update({
        reconciles: [
          {
            family: feed,
            params: () => ({}),
            fallback: "refetchRequired",
            summary: { kind: "summary", summary: "visibleCount" },
          },
        ],
        load: ({ body }) => ({ visibleCount: body.visibleCount }),
      });

    const saveLine = saveStats.line({ body: { visibleCount: 2 } });
    const plan = saveLine.mutationResponse();

    assert.deepEqual(feedLine.value(), {
      items: [{ id: "t1", title: "First" }],
      cursor: "next",
      visibleCount: 2,
    });
    assert.equal(plan.targets[0].line.familyKind, "paged");
    assert.equal(plan.executionArtifacts[0].kind, "exactSummary");
    assert.equal(plan.executionArtifacts[0].familyKind, "paged");
    assert.equal(plan.executionArtifacts[0].summary, "visibleCount");
    assert.equal(plan.executionArtifacts[0].summaryScope, "pageWindow");
    assert.equal(feedLine.diagnostics().lastDeliveryScope, "summary");
    assert.equal(feedLine.diagnostics().lastPatchedSummary, "visibleCount");
    assert.equal(
      saveLine.summary().diagnostics.latest.mutationResponseExecutionDigest,
      plan.executionDigest,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("collection and summary mutation reconciliation deny malformed target declarations", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const detailRead = runtime.signals.api({}).url("/users/:userId").detail({
      load: ({ userId }) => ({ id: userId }),
    });
    const collectionResponse = runtime.signals.resource.response.array({
      itemId: (item) => item.id,
    });
    const taskList = runtime.signals.api({}).url("/tasks").response(collectionResponse).list({
      load: () => [{ id: "t1" }],
    });
    const summarizedTaskList = runtime.signals.api({}).url("/task-search")
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
          items: [{ id: "t1" }],
          total: 1,
        }),
      });

    assert.throws(
      () =>
        runtime.signals.api({}).url("/users/:userId")
          .response(runtime.signals.resource.response.detail()())
          .update({
            reconciles: [
              {
                family: detailRead,
                params: ({ userId }) => ({ userId }),
                fallback: "refetchRequired",
                collection: { kind: "item" },
              },
            ],
            load: ({ userId }) => ({ id: userId }),
          }),
      /collection item reconciliation requires a collection or paged read family/,
    );
    assert.throws(
      () =>
        runtime.signals.api({}).url("/task-search/stats")
          .response(runtime.signals.resource.response.detail()({ total: "total" }))
          .update({
            reconciles: [
              {
                family: taskList,
                params: () => ({}),
                fallback: "refetchRequired",
                summary: { kind: "summary", summary: "total" },
              },
            ],
            load: ({ body }) => ({ total: body.total }),
          }),
      /summary "total" is not declared on the target family/,
    );
    assert.throws(
      () =>
        runtime.signals.api({}).url("/task-search/stats")
          .response(runtime.signals.resource.response.detail()())
          .update({
            reconciles: [
              {
                family: summarizedTaskList,
                params: () => ({}),
                fallback: "refetchRequired",
                summary: { kind: "summary", summary: "total" },
              },
            ],
            load: () => ({ total: 2 }),
          }),
      /summary "total" is not declared on the mutation response lens/,
    );
    assert.throws(
      () =>
        runtime.signals.api({}).url("/tasks")
          .response(runtime.signals.resource.response.detail()())
          .create({
            reconciles: [
              {
                family: taskList,
                params: () => ({}),
                fallback: "refetchRequired",
                collection: { kind: "item" },
              },
            ],
            load: ({ body }) => body,
          }),
      /collection item replacement is currently admitted only for update\/save and remove\/delete responses/,
    );
  } finally {
    await runtime.cleanup();
  }
});
