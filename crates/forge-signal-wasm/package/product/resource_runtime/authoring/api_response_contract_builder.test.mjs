import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";
import { normalizeRouteLineArtifact } from "./api_route/route_line_artifact_proof.mjs";

test("api.url(...).response(array contract).list(...) lowers typed object aspects into patchable collection truth", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const taskResponse = signals.resource.response.array({
      itemId: (task) => task.id,
      aspects: signals.resource.response.objectAspects()({
        title: "title",
        status: "status",
      }),
    });
    const tasks = signals.api({}).url("/workspaces/:workspaceId/tasks")
      .response(taskResponse)
      .list({
        load: ({ workspaceId }) => [
          { id: `${workspaceId}:1`, title: "First", status: "open" },
        ],
      });
    const rawTasks = signals.resource.collection({
      params: signalsMod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        signalsMod.resourceParamIdentity(
          { workspaceId },
          `/workspaces/${encodeURIComponent(String(workspaceId))}/tasks`,
        ),
      itemIdentity: (task) => task.id,
      reconcile: signalsMod.resourceCollectionShape({
        items: (value) => value,
        replaceItems: (_value, nextItems) => [...nextItems],
        aspects: signalsMod.resourceItemAspects({
          title: {
            read: (task) => task.title,
            write: (task, title) => ({ ...task, title }),
          },
          status: {
            read: (task) => task.status,
            write: (task, status) => ({ ...task, status }),
          },
        }),
      }),
      load: ({ workspaceId }) => [
        { id: `${workspaceId}:1`, title: "First", status: "open" },
      ],
    });

    assert.deepEqual(
      normalizeRouteLineArtifact(tasks.line({ workspaceId: "demo" })),
      normalizeRouteLineArtifact(rawTasks.line({ workspaceId: "demo" })),
    );
    assert.equal(typeof tasks.patch.item, "function");
    assert.equal(typeof tasks.patch.itemAspect, "function");
    assert.equal(typeof tasks.delivery.itemAspect, "function");

    const line = tasks.line({ workspaceId: "demo" });
    const result = line.patch(
      tasks.patch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Renamed",
      }),
    );

    assert.deepEqual(result, {
      kind: "narrowed",
      scope: "aspect",
      itemId: "demo:1",
      aspect: "title",
    });
    assert.deepEqual(line.value(), [
      { id: "demo:1", title: "Renamed", status: "open" },
    ]);
    assert.equal(Object.isFrozen(line.value()[0]), false);
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).response(objectItems contract).list(...) patches typed envelope responses", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    const taskResponse = signals.resource.response.objectItems()({
      field: "tasks",
      itemId: (task) => task.id,
      aspects: signals.resource.response.objectAspects()({
        title: "title",
      }),
    });
    const tasks = signals.api({}).url("/task-page")
      .response(taskResponse)
      .list({
        load: () => ({
          tasks: [{ id: "t1", title: "First" }],
          nextCursor: "cursor-2",
        }),
      });

    const line = tasks.line({});
    const result = line.patch(
      tasks.patch.itemAspect({
        itemId: "t1",
        aspect: "title",
        value: "Renamed",
      }),
    );

    assert.deepEqual(result, {
      kind: "narrowed",
      scope: "aspect",
      itemId: "t1",
      aspect: "title",
    });
    assert.deepEqual(line.value(), {
      tasks: [{ id: "t1", title: "Renamed" }],
      nextCursor: "cursor-2",
    });
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).response(collection contract).list(...) patches arbitrary typed response shapes", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    const taskResponse = signals.resource.response.collection()({
      itemId: (task) => task.id,
      items: (value) => value.edges.map((edge) => edge.node),
      replaceItems: (value, nextItems) => ({
        ...value,
        edges: nextItems.map((node) => ({ node })),
      }),
      aspects: signals.resource.response.objectAspects()({
        status: "status",
      }),
    });
    const tasks = signals.api({}).url("/task-connection")
      .response(taskResponse)
      .list({
        load: () => ({
          edges: [{ node: { id: "t1", status: "open" } }],
          pageInfo: { hasNextPage: false },
        }),
      });

    const line = tasks.line({});
    const result = line.patch(
      tasks.patch.itemAspect({
        itemId: "t1",
        aspect: "status",
        value: "done",
      }),
    );

    assert.deepEqual(result, {
      kind: "narrowed",
      scope: "aspect",
      itemId: "t1",
      aspect: "status",
    });
    assert.deepEqual(line.value(), {
      edges: [{ node: { id: "t1", status: "done" } }],
      pageInfo: { hasNextPage: false },
    });
  } finally {
    await runtime.cleanup();
  }
});

test("response contract lanes own identity and reconciliation boundaries", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskResponse = runtime.signals.resource.response.array({
      itemId: (task) => task.id,
    });
    const responseLane = runtime.signals.api({}).url("/tasks").response(taskResponse);

    assert.equal("items" in responseLane, false);
    assert.equal("reconcile" in responseLane, false);
    assert.equal("aspect" in responseLane, false);
    assert.equal("summary" in responseLane, false);
    assert.equal("pageWindowSummary" in responseLane, false);
    assert.throws(
      () => responseLane.detail({ load: () => ({ id: "t1" }) }),
      /response\(\.\.\.\) is a collection response lane/,
    );
    assert.throws(
      () => responseLane.create({ load: () => ({ id: "t1" }) }),
      /response\(\.\.\.\) is a collection response lane/,
    );
    assert.throws(
      () => responseLane.update({ load: () => ({ id: "t1" }) }),
      /response\(\.\.\.\) is a collection response lane/,
    );
    assert.throws(
      () => responseLane.remove({ load: () => ({ id: "t1" }) }),
      /response\(\.\.\.\) is a collection response lane/,
    );
    assert.throws(
      () =>
        runtime.signals.api({}).url("/tasks").response(taskResponse).list({
          itemIdentity: (task) => task.id,
          load: () => [{ id: "t1" }],
        }),
      /response\(\.\.\.\) owns itemIdentity/,
    );
    assert.throws(
      () =>
        runtime.signals.api({}).url("/tasks").response(taskResponse).list({
          reconcile: runtime.signalsMod.resourceCollectionShape({
            items: (value) => value,
            replaceItems: (_value, nextItems) => [...nextItems],
          }),
          load: () => [{ id: "t1" }],
        }),
      /response\(\.\.\.\) owns reconcile/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("response contracts deny malformed declarations and non-array response values", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    assert.throws(
      () => runtime.signals.resource.response.objectAspects()(null),
      /requires an aspect field object/,
    );
    assert.throws(
      () =>
        runtime.signals.resource.response.objectAspects()({
          title: "",
        }),
      /requires a non-empty field name/,
    );
    assert.throws(
      () => runtime.signals.resource.response.array({}),
      /requires itemId/,
    );
    assert.throws(
      () =>
        runtime.signals.resource.response.array({
          itemId: (task) => task.id,
          aspects: {},
        }),
      /resource.response.array\(\.\.\.\) requires aspects created with resourceItemAspects/,
    );
    assert.throws(
      () =>
        runtime.signals.resource.response.collection({
          itemId: (task) => task.id,
          items: (value) => value.tasks,
        }),
      /resource.response.collection\(\.\.\.\) requires replaceItems/,
    );
    assert.throws(
      () =>
        runtime.signals.resource.response.objectItems()({
          field: "",
          itemId: (task) => task.id,
        }),
      /requires a non-empty field name/,
    );
    assert.throws(
      () => runtime.signals.api({}).url("/tasks").response({}),
      /requires a resource.response collection contract/,
    );

    const response = runtime.signals.resource.response.array({
      itemId: (task) => task.id,
    });
    const tasks = runtime.signals.api({}).url("/tasks").response(response).list({
      load: () => ({ id: "not-an-array" }),
    });

    assert.throws(
      () =>
        tasks.line({}).patch(
          tasks.patch.item({
            itemId: "t1",
            nextItem: { id: "t1" },
          }),
        ),
      /resource.response.array\(\.\.\.\) requires list\/paged values to stay direct arrays/,
    );

    const badItemsResponse = runtime.signals.resource.response.collection({
      itemId: (task) => task.id,
      items: () => ({ not: "items" }),
      replaceItems: (value) => value,
    });
    const badItems = runtime.signals.api({}).url("/bad-items")
      .response(badItemsResponse)
      .list({
        load: () => ({ tasks: [{ id: "t1" }] }),
      });
    assert.throws(
      () =>
        badItems.line({}).patch(
          badItems.patch.item({
            itemId: "t1",
            nextItem: { id: "t1" },
          }),
        ),
      /resource.response.collection\(\.\.\.\) requires items\(value\) to produce an array/,
    );

    const badReplaceResponse = runtime.signals.resource.response.collection({
      itemId: (task) => task.id,
      items: (value) => value.tasks,
      replaceItems: () => ({ tasks: { not: "items" } }),
    });
    const badReplace = runtime.signals.api({}).url("/bad-replace")
      .response(badReplaceResponse)
      .list({
        load: () => ({ tasks: [{ id: "t1" }] }),
      });
    assert.throws(
      () =>
        badReplace.line({}).patch(
          badReplace.patch.item({
            itemId: "t1",
            nextItem: { id: "t1" },
          }),
        ),
      /resource.response.collection\(\.\.\.\) requires replaceItems\(value, nextItems\) to produce an array/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("object aspect response contracts deny non-object item patch writes before mutation", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const response = runtime.signals.resource.response.array({
      itemId: (task) => String(task),
      aspects: runtime.signals.resource.response.objectAspects()({
        title: "title",
      }),
    });
    const tasks = runtime.signals.api({}).url("/tasks").response(response).list({
      load: () => ["not-object"],
    });
    const line = tasks.line({});
    assert.throws(
      () =>
        line.patch(
          tasks.patch.itemAspect({
            itemId: "not-object",
            aspect: "title",
            value: "Updated",
          }),
        ),
      /requires object items/,
    );
    assert.deepEqual(line.value(), ["not-object"]);
  } finally {
    await runtime.cleanup();
  }
});
