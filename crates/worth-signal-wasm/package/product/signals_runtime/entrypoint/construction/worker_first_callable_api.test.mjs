import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

async function settleWorkerResourceLine() {
  await Promise.resolve();
  await new Promise((resolve) => setTimeout(resolve, 0));
}

test("default worker-first root exposes terminal api detail, collection, and paged families", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });

  try {
    const workerSignals = await createSignals();

    const api = workerSignals.api({
      baseUrl: "https://example.test",
      effects: workerSignals.resource.effects.branchNative(),
    });
    const scopedApi = workerSignals.scope("wizard").api({
      headers: { "x-scope": "wizard" },
    });

    const route = api.url("/tasks/:taskId").response(
      workerSignals.resource.response.detail()({ title: "title" }),
    );
    const scopedRoute = scopedApi.url("/tasks/:taskId").response(
      workerSignals.resource.response.detail()({ title: "title" }),
    );

    const detailLine = route.detail({
      load: ({ taskId }) => ({ id: taskId, title: "Draft" }),
    }).line({ taskId: "task-1" });
    await settleWorkerResourceLine();
    assert.deepEqual(detailLine.value(), {
      id: "task-1",
      title: "Draft",
    });

    const scopedDetailLine = scopedRoute.detail({
      load: ({ taskId }) => ({ id: taskId, title: "Scoped" }),
    }).line({ taskId: "task-2" });
    await settleWorkerResourceLine();
    assert.deepEqual(scopedDetailLine.value(), {
      id: "task-2",
      title: "Scoped",
    });

    const collectionLine = api.url("/tasks").list({
      itemIdentity: (item) => item.id,
      load: () => [{ id: "task-1", title: "List" }],
    }).line({});
    await settleWorkerResourceLine();
    assert.deepEqual(collectionLine.value(), [{
      id: "task-1",
      title: "List",
    }]);

    const pagedLine = api.url("/feed").paged({
      itemIdentity: (item) => item.id,
      accumulatePage: (existing, next) => [...existing, ...next],
      load: () => [{ id: "feed-1", title: "Paged" }],
    }).line({});
    await settleWorkerResourceLine();
    assert.deepEqual(pagedLine.value(), [{
      id: "feed-1",
      title: "Paged",
    }]);

    detailLine.free();
    scopedDetailLine.free();
    collectionLine.free();
    pagedLine.free();
    await workerSignals.terminate();
  } finally {
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("api families expose optionalLine and execute as first-class final-form lanes", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });

  try {
    const workerSignals = await createSignals();
    let detailCallCount = 0;
    const route = workerSignals.api({
      baseUrl: "https://example.test",
      effects: workerSignals.resource.effects.branchNative(),
    }).url("/tasks/:taskId").response(
      workerSignals.resource.response.detail()({ title: "title" }),
    );
    const detail = route.detail({
      load: ({ taskId }) => ({ id: taskId, title: `Task ${++detailCallCount}` }),
    });

    assert.equal(detail.optionalLine({ enabled: false }), null);

    const resident = detail.optionalLine({ taskId: "task-1" });
    await settleWorkerResourceLine();
    assert.deepEqual(resident?.value(), {
      id: "task-1",
      title: "Task 1",
    });

    const execution = detail.execute({ taskId: "task-2" });
    const settlement = await execution.settled();

    assert.equal(settlement.resultKind, "fulfilled");
    assert.deepEqual(settlement.value, {
      id: "task-2",
      title: "Task 2",
    });
    await workerSignals.terminate();
  } finally {
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
