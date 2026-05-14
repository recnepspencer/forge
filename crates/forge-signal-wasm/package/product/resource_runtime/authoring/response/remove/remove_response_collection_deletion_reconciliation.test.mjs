import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("remove responses can delete resident collection items and patch declared summaries in one canonical plan", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskList = runtime.signals.api({}).url("/tasks")
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
    const taskLine = taskList.line({});
    const removeTask = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()({ total: "total" }))
      .remove({
        reconciles: [
          {
            family: taskList,
            params: () => ({}),
            fallback: "deletionUnavailable",
            collection: { kind: "delete" },
          },
          {
            family: taskList,
            params: () => ({}),
            fallback: "refetchRequired",
            summary: { kind: "summary", summary: "total" },
          },
        ],
        load: ({ taskId }) => ({ id: taskId, total: 1 }),
      });

    const removeLine = removeTask.line({ taskId: "t1" });
    const plan = removeLine.mutationResponse();

    assert.deepEqual(taskLine.value(), {
      items: [{ id: "t2", title: "Second" }],
      total: 1,
    });
    assert.equal(plan.targets[0].fallback.kind, "deletionUnavailable");
    assert.equal(plan.targets[0].reconciliation.kind, "delete");
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionDelete");
    assert.equal(plan.executionArtifacts[0].itemId, "t1");
    assert.equal(plan.executionArtifacts[0].deliveryKind, "patch");
    assert.equal(plan.executionArtifacts[0].deliveryScope, "item");
    assert.equal(plan.executionArtifacts[1].kind, "exactSummary");
    assert.equal(plan.executionArtifacts[1].summary, "total");
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.equal(plan.counters.appliedTargetBreadth, 2);
    assert.equal(taskLine.diagnostics().lastDeliveryScope, "summary");
    assert.equal(taskLine.diagnostics().lastPatchedSummary, "total");
    assert.equal(taskLine.history().lifecycle.at(-1)?.event, "delivered");
  } finally {
    await runtime.cleanup();
  }
});

test("remove responses preserve typed deletionUnavailable fallback when the target line is not resident", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskList = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.array({
        itemId: (item) => item.id,
      }))
      .list({
        load: () => [],
      });
    const plan = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .remove({
        reconciles: [
          {
            family: taskList,
            params: () => ({}),
            fallback: "deletionUnavailable",
            collection: { kind: "delete" },
          },
        ],
        load: ({ taskId }) => ({ id: taskId }),
      })
      .line({ taskId: "t2" })
      .mutationResponse();

    assert.equal(plan.executionArtifacts[0].kind, "fallback");
    assert.equal(plan.executionArtifacts[0].fallback, "deletionUnavailable");
    assert.equal(plan.confirmation.kind, "partialCanonicalTruth");
    assert.equal(plan.counters.fallbackBreadth, 1);
  } finally {
    await runtime.cleanup();
  }
});

test("remove responses classify pure exact deletion as consumed canonical truth", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskList = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.array({
        itemId: (item) => item.id,
      }))
      .list({
        load: () => [
          { id: "t1", title: "First" },
          { id: "t2", title: "Second" },
        ],
      });
    taskList.line({});
    const plan = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .remove({
        reconciles: [
          {
            family: taskList,
            params: () => ({}),
            fallback: "deletionUnavailable",
            collection: { kind: "delete" },
          },
        ],
        load: ({ taskId }) => ({ id: taskId }),
      })
      .line({ taskId: "t1" })
      .mutationResponse();

    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionDelete");
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.equal(plan.confirmation.exactTargetCount, 1);
    assert.equal(plan.confirmation.fallbackTargetCount, 0);
  } finally {
    await runtime.cleanup();
  }
});

test("remove response deletion still denies malformed collection declarations", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const directTasks = runtime.signals.api({}).url("/tasks")
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
            "page-1": [{ id: "t1", page: "page-1", title: "First" }],
          },
        }),
      });

    assert.throws(
      () =>
        runtime.signals.api({}).url("/tasks/:taskId")
          .response(runtime.signals.resource.response.detail()())
          .remove({
            reconciles: [
              {
                family: directTasks,
                params: () => ({}),
                fallback: "deletionUnavailable",
                collection: { kind: "insert", placement: "append" },
              },
            ],
            load: ({ taskId }) => ({ id: taskId }),
          }),
      /collection insert reconciliation is currently admitted only for create responses/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("remove responses can delete grouped and named collection items while preserving empty sibling buckets", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const groupedTasks = runtime.signals.api({}).url("/grouped-tasks")
      .response(runtime.signals.resource.response.grouped()({
        itemId: (task) => task.id,
        groupId: (task) => task.group,
        groupForItem: () => "todo",
        groups: (value) => value.groups,
        replaceGroups: (value, groups) => ({ ...value, groups }),
        replaceGroupItem: (value, groupId, itemId, nextItem) => ({
          ...value,
          groups: Object.fromEntries(
            Object.entries(value.groups).map(([key, items]) => [
              key,
              key === groupId
                ? items.map((item) => item.id === itemId ? nextItem : item)
                : items,
            ]),
          ),
        }),
      }))
      .list({
        load: () => ({
          groups: {
            todo: [
              { id: "task:1", group: "todo", title: "First" },
              { id: "task:2", group: "todo", title: "Second" },
            ],
            done: [],
          },
        }),
      });
    const namedTasks = runtime.signals.api({}).url("/named-tasks")
      .response(runtime.signals.resource.response.named()({
        itemId: (task) => task.id,
        collectionId: (task) => task.collection,
        collectionForItem: () => "backlog",
        collections: (value) => value.collections,
        replaceCollections: (value, collections) => ({ ...value, collections }),
        replaceCollectionItem: (value, collectionId, itemId, nextItem) => ({
          ...value,
          collections: Object.fromEntries(
            Object.entries(value.collections).map(([key, items]) => [
              key,
              key === collectionId
                ? items.map((item) => item.id === itemId ? nextItem : item)
                : items,
            ]),
          ),
        }),
      }))
      .list({
        load: () => ({
          collections: {
            backlog: [
              { id: "task:1", collection: "backlog", title: "First" },
              { id: "task:2", collection: "backlog", title: "Second" },
            ],
            active: [],
          },
        }),
      });
    const groupedLine = groupedTasks.line({});
    const namedLine = namedTasks.line({});

    const groupedPlan = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .remove({
        reconciles: [{
          family: groupedTasks,
          params: () => ({}),
          fallback: "deletionUnavailable",
          collection: { kind: "delete" },
        }],
        load: ({ taskId }) => ({ id: taskId }),
      })
      .line({ taskId: "task:1" })
      .mutationResponse();
    const namedPlan = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .remove({
        reconciles: [{
          family: namedTasks,
          params: () => ({}),
          fallback: "deletionUnavailable",
          collection: { kind: "delete" },
        }],
        load: ({ taskId }) => ({ id: taskId }),
      })
      .line({ taskId: "task:1" })
      .mutationResponse();

    assert.deepEqual(groupedTasks.line({}).value(), {
      groups: {
        todo: [{ id: "task:2", group: "todo", title: "Second" }],
        done: [],
      },
    });
    assert.equal(groupedPlan.targets[0].line.residency, "resident");
    assert.equal(groupedPlan.executionArtifacts[0].kind, "exactCollectionDelete");
    assert.deepEqual(groupedLine.diagnostics().lastEffect.locusProof.cost, {
      lookup: "group-key-item-id",
      lookupBreadth: 1,
      traversal: "single-group",
      traversalBreadth: 1,
      reconstruction: "replaceGroups",
      reconstructionBreadth: 1,
    });

    assert.deepEqual(namedTasks.line({}).value(), {
      collections: {
        backlog: [{ id: "task:2", collection: "backlog", title: "Second" }],
        active: [],
      },
    });
    assert.equal(namedPlan.targets[0].line.residency, "resident");
    assert.equal(namedPlan.executionArtifacts[0].kind, "exactCollectionDelete");
    assert.deepEqual(namedLine.diagnostics().lastEffect.locusProof.cost, {
      lookup: "collection-key-item-id",
      lookupBreadth: 1,
      traversal: "single-named-collection",
      traversalBreadth: 1,
      reconstruction: "replaceCollections",
      reconstructionBreadth: 1,
    });
  } finally {
    await runtime.cleanup();
  }
});
