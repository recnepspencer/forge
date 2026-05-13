import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("create responses can insert resident collection items through declared append placement", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskList = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.array({
        itemId: (item) => item.id,
      }))
      .list({
        load: () => [{ id: "t1", title: "First" }],
      });
    const taskLine = taskList.line({});
    const createTask = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        reconciles: [
          {
            family: taskList,
            params: () => ({}),
            fallback: "placementUnavailable",
            collection: { kind: "insert", placement: "append" },
          },
        ],
        load: ({ body }) => ({ id: body.id, title: body.title }),
      });

    const plan = createTask.line({
      body: { id: "t2", title: "Second" },
    }).mutationResponse();

    assert.deepEqual(taskLine.value(), [
      { id: "t1", title: "First" },
      { id: "t2", title: "Second" },
    ]);
    assert.equal(plan.targets[0].fallback.kind, "placementUnavailable");
    assert.equal(plan.targets[0].reconciliation.kind, "insert");
    assert.equal(plan.targets[0].reconciliation.placement, "append");
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionInsert");
    assert.equal(plan.executionArtifacts[0].placement, "append");
    assert.equal(plan.executionArtifacts[0].itemId, "t2");
    assert.equal(plan.executionArtifacts[0].deliveryKind, "patch");
    assert.equal(plan.executionArtifacts[0].deliveryScope, "item");
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.equal(taskLine.diagnostics().lastPatchKind, "insert");
    assert.equal(taskLine.diagnostics().lastDeliveryScope, "item");
    assert.equal(taskLine.diagnostics().lastPatchedItemId, "t2");
    assert.equal(taskLine.diagnostics().lastEffect.patch.kind, "insert");
    assert.equal(plan.counters.appliedTargetBreadth, 1);
  } finally {
    await runtime.cleanup();
  }
});

test("create responses can fan out exact insert and summary targets in one plan", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskList = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.array({
        itemId: (item) => item.id,
      }))
      .list({
        load: () => [{ id: "t1", title: "First", total: 1 }],
      });
    const summaryList = runtime.signals.api({}).url("/task-search")
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
          items: [{ id: "t1", title: "First", total: 1 }],
          total: 1,
        }),
      });
    const taskLine = taskList.line({});
    const summaryLine = summaryList.line({});
    const createTask = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()({ total: "total" }))
      .create({
        reconciles: [
          {
            family: taskList,
            params: () => ({}),
            fallback: "placementUnavailable",
            collection: { kind: "insert", placement: "prepend" },
          },
          {
            family: summaryList,
            params: () => ({}),
            fallback: "refetchRequired",
            summary: { kind: "summary", summary: "total" },
          },
        ],
        load: ({ body }) => ({
          id: body.id,
          title: body.title,
          total: body.total,
        }),
      });

    const plan = createTask.line({
      body: { id: "t2", title: "Second", total: 2 },
    }).mutationResponse();

    assert.deepEqual(taskLine.value(), [
      { id: "t2", title: "Second", total: 2 },
      { id: "t1", title: "First", total: 1 },
    ]);
    assert.equal(summaryLine.value().total, 2);
    assert.equal(plan.executionArtifacts.length, 2);
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionInsert");
    assert.equal(plan.executionArtifacts[1].kind, "exactSummary");
    assert.equal(plan.executionArtifacts[1].summary, "total");
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.equal(plan.counters.executionBreadth, 2);
    assert.equal(plan.counters.appliedTargetBreadth, 2);
  } finally {
    await runtime.cleanup();
  }
});

test("create responses preserve typed placementUnavailable fallback when a target line is not resident", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskList = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.array({
        itemId: (item) => item.id,
      }))
      .list({
        load: () => [],
      });
    const plan = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        reconciles: [
          {
            family: taskList,
            params: () => ({}),
            fallback: "placementUnavailable",
            collection: { kind: "insert", placement: "append" },
          },
        ],
        load: ({ body }) => ({ id: body.id, title: body.title }),
      })
      .line({
        body: { id: "t2", title: "Second" },
      })
      .mutationResponse();

    assert.equal(plan.executionArtifacts[0].kind, "fallback");
    assert.equal(plan.executionArtifacts[0].fallback, "placementUnavailable");
    assert.equal(plan.confirmation.kind, "partialCanonicalTruth");
    assert.equal(plan.counters.fallbackBreadth, 1);
  } finally {
    await runtime.cleanup();
  }
});

test("create response placement denies unsupported replacement and malformed placement declarations", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskList = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.array({
        itemId: (item) => item.id,
      }))
      .list({
        load: () => [{ id: "t1", title: "First" }],
      });

    assert.throws(
      () =>
        runtime.signals.api({}).url("/tasks")
          .response(runtime.signals.resource.response.detail()())
          .create({
            reconciles: [
              {
                family: taskList,
                params: () => ({}),
                fallback: "placementUnavailable",
                collection: { kind: "item" },
              },
            ],
            load: ({ body }) => body,
          }),
      /collection item replacement is currently admitted only for update\/save and remove\/delete responses/,
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
                fallback: "placementUnavailable",
                collection: { kind: "insert", placement: "middle" },
              },
            ],
            load: ({ body }) => body,
          }),
      /collection\.placement must be append or prepend/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("create response placement admits sparse-page collection topologies through exact page-local insertion", async () => {
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
            "page-1": [{ id: "t1", page: "page-1", title: "First" }],
            "page-2": [{ id: "t9", page: "page-2", title: "Sibling" }],
          },
        }),
      });
    const line = sparseTasks.line({});
    const createTask = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        reconciles: [{
          family: sparseTasks,
          params: () => ({}),
          fallback: "placementUnavailable",
          collection: { kind: "insert", placement: "append" },
        }],
        load: ({ body }) => body,
      });

    const plan = createTask.line({
      body: { id: "t2", page: "page-1", title: "Second" },
    }).mutationResponse();

    assert.deepEqual(line.value(), {
      pages: {
        "page-1": [
          { id: "t1", page: "page-1", title: "First" },
          { id: "t2", page: "page-1", title: "Second" },
        ],
        "page-2": [{ id: "t9", page: "page-2", title: "Sibling" }],
      },
    });
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionInsert");
    assert.equal(plan.executionArtifacts[0].placement, "append");
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
  } finally {
    await runtime.cleanup();
  }
});
