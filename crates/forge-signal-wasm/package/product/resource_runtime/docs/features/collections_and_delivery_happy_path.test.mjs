import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";

test("collections and delivery doc happy path covers direct-array patch helpers and reconcile summaries", async () => {
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
    assert.equal(patchResult.scope, "aspect");
    assert.equal(catalogLine.value().total, 2);
    assert.equal(deliveryResult.scope, "summary");
  } finally {
    await runtime.cleanup();
  }
});
