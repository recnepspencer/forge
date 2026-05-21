import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("default worker-first root exposes declarative api namespaces and keeps resource-family realization explicit", async () => {
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

    assert.equal(typeof api.scope, "function");
    assert.equal(typeof api.url, "function");
    assert.equal(typeof scopedApi.scope, "function");

    const route = api.url("/tasks/:taskId").response(
      workerSignals.resource.response.detail()({ title: "title" }),
    );
    const scopedRoute = scopedApi.url("/tasks/:taskId").response(
      workerSignals.resource.response.detail()({ title: "title" }),
    );

    assert.equal(typeof route.detail, "function");
    assert.equal(typeof route.update, "function");
    assert.equal(typeof route.remove, "function");
    assert.equal(typeof scopedRoute.detail, "function");

    assert.throws(
      () => route.detail({
        load: ({ taskId }) => ({ id: taskId, title: "Draft" }),
      }),
      /worker-first resource surface/i,
    );
    assert.throws(
      () => scopedRoute.detail({
        load: ({ taskId }) => ({ id: taskId, title: "Draft" }),
      }),
      /worker-first resource surface/i,
    );
    assert.throws(
      () => api.url("/tasks").list({
        load: () => ({ tasks: [] }),
      }),
      /worker-first resource surface/i,
    );
  } finally {
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
