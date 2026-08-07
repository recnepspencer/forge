import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("createSignals admits workerFirst assets only when both URLs are present", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { planCreateSignalsDeployment, cleanup } = await loadSignalsModule();
  try {
    const wasmUrl = new URL("https://example.test/worth_signal_wasm_bg.wasm");
    const workerUrl = new URL("https://example.test/worker.js");
    const plan = planCreateSignalsDeployment({
      assets: { wasmUrl, workerUrl },
    });
    assert.equal(plan.family, "workerFirst");
    assert.equal(plan.request.assets.wasmUrl.href, wasmUrl.href);
    assert.equal(plan.request.assets.workerUrl.href, workerUrl.href);
  } finally {
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("createSignals rejects partial workerFirst assets", async () => {
  const { planCreateSignalsDeployment, cleanup } = await loadSignalsModule();
  try {
    assert.throws(
      () => planCreateSignalsDeployment({
        assets: { wasmUrl: "https://example.test/a.wasm" },
      }),
      /both wasmUrl and workerUrl/u,
    );
    assert.throws(
      () => planCreateSignalsDeployment({
        assets: { workerUrl: "https://example.test/worker.js" },
      }),
      /both wasmUrl and workerUrl/u,
    );
  } finally {
    await cleanup();
  }
});

test("createSignals mainThreadCompatibility accepts wasmUrl only", async () => {
  const { planCreateSignalsDeployment, cleanup } = await loadSignalsModule();
  try {
    const plan = planCreateSignalsDeployment({
      deployment: "mainThreadCompatibility",
      assets: { wasmUrl: "https://example.test/a.wasm" },
    });
    assert.equal(plan.family, "mainThreadCompatibility");
    assert.equal(plan.request.assets.wasmUrl.href, "https://example.test/a.wasm");
    assert.equal(plan.request.assets.workerUrl, null);
    assert.throws(
      () => planCreateSignalsDeployment({
        deployment: "mainThreadCompatibility",
        assets: {
          wasmUrl: "https://example.test/a.wasm",
          workerUrl: "https://example.test/worker.js",
        },
      }),
      /workerUrl is only valid/u,
    );
  } finally {
    await cleanup();
  }
});
