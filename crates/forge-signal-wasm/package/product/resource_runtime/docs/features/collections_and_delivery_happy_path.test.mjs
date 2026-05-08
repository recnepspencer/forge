import assert from "node:assert/strict";
import fs from "node:fs";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";

test("collections and delivery doc happy path covers response contracts, direct patch helpers, and reconcile summaries", async () => {
  const doc = fs.readFileSync(
    "crates/forge-signal-wasm/docs/feature_collections_and_delivery.md",
    "utf8",
  );

  assert.match(doc, /resource\.response\.array\(\.\.\.\)/);
  assert.match(doc, /resource\.response\.objectItems<T>\(\)\(\.\.\.\)/);
  assert.match(doc, /resource\.response\.collection<T>\(\)\(\.\.\.\)/);
  assert.match(doc, /resource\.response\.objectAspects<T>\(\)\(\.\.\.\)/);

  const runtime = await createRealRequestRuntime();
  try {
    const api = runtime.signals.api({});
    const tasks = api.url("/tasks")
      .items((item) => item.id)
      .aspect(
        "title",
        (item) => item.title,
        (item, title) => ({ ...item, title }),
      )
      .list({
        load: () => [{ id: "t1", title: "First" }],
      });
    const response = runtime.signals.resource.response.array({
      itemId: (item) => item.id,
      aspects: runtime.signals.resource.response.objectAspects()({
        title: "title",
      }),
    });
    const responseTasks = api.url("/response-tasks")
      .response(response)
      .list({
        load: () => [{ id: "rt1", title: "First" }],
      });
    const envelopeResponse = runtime.signals.resource.response.objectItems()({
      field: "tasks",
      itemId: (item) => item.id,
      aspects: runtime.signals.resource.response.objectAspects()({
        title: "title",
      }),
    });
    const envelopeTasks = api.url("/response-task-page")
      .response(envelopeResponse)
      .list({
        load: () => ({
          tasks: [{ id: "et1", title: "First" }],
          nextCursor: null,
        }),
      });
    const connectionResponse = runtime.signals.resource.response.collection({
      itemId: (item) => item.id,
      items: (value) => value.edges.map((edge) => edge.node),
      replaceItems: (value, nextItems) => ({
        ...value,
        edges: nextItems.map((node) => ({ node })),
      }),
      aspects: runtime.signals.resource.response.objectAspects()({
        title: "title",
      }),
    });
    const connectionTasks = api.url("/response-task-connection")
      .response(connectionResponse)
      .list({
        load: () => ({
          edges: [{ node: { id: "ct1", title: "First" } }],
          pageInfo: { hasNextPage: false },
        }),
      });
    const catalog = api.url("/catalog")
      .items((item) => item.id)
      .reconcile(
        (value) => value.items,
        (value, nextItems) => ({ ...value, items: [...nextItems] }),
      )
      .summary(
        "total",
        (value) => value.total,
        (value, total) => ({ ...value, total }),
      )
      .list({
        load: () => ({ items: [{ id: "t1", title: "First" }], total: 1 }),
      });

    const taskLine = tasks.line({});
    const responseTaskLine = responseTasks.line({});
    const envelopeTaskLine = envelopeTasks.line({});
    const connectionTaskLine = connectionTasks.line({});
    const responsePatchResult = responseTaskLine.patch(
      responseTasks.patch.itemAspect({
        itemId: "rt1",
        aspect: "title",
        value: "Response Updated",
      }),
    );
    const envelopePatchResult = envelopeTaskLine.patch(
      envelopeTasks.patch.itemAspect({
        itemId: "et1",
        aspect: "title",
        value: "Envelope Updated",
      }),
    );
    const connectionPatchResult = connectionTaskLine.patch(
      connectionTasks.patch.itemAspect({
        itemId: "ct1",
        aspect: "title",
        value: "Connection Updated",
      }),
    );
    const patchResult = taskLine.patch(
      tasks.patch.itemAspect({
        itemId: "t1",
        aspect: "title",
        value: "Updated",
      }),
    );
    const catalogLine = catalog.line({});
    const deliveryResult = catalogLine.deliver(
      catalog.delivery.summary({
        packetId: "pkt-1",
        basisId: null,
        nextBasisId: "basis-1",
        summary: "total",
        value: 2,
      }),
    );

    assert.equal(taskLine.value()[0].title, "Updated");
    assert.equal(responseTaskLine.value()[0].title, "Response Updated");
    assert.equal(envelopeTaskLine.value().tasks[0].title, "Envelope Updated");
    assert.equal(
      connectionTaskLine.value().edges[0].node.title,
      "Connection Updated",
    );
    assert.equal(responsePatchResult.scope, "aspect");
    assert.equal(envelopePatchResult.scope, "aspect");
    assert.equal(connectionPatchResult.scope, "aspect");
    assert.equal(patchResult.scope, "aspect");
    assert.equal(catalogLine.value().total, 2);
    assert.equal(deliveryResult.scope, "summary");
  } finally {
    await runtime.cleanup();
  }
});
