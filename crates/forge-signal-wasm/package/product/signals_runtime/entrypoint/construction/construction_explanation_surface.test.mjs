import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("explainCreateSignalsConstruction exposes the selected compatibility lane explicitly", async () => {
  const { explainCreateSignalsConstruction, cleanup } = await loadSignalsModule();
  try {
    const explanation = explainCreateSignalsConstruction({
      deployment: "mainThreadCompatibility",
    });
    assert.deepEqual(explanation, {
      requestedDeployment: "mainThreadCompatibility",
      selectedFamily: "mainThreadCompatibility",
      selectedDeployment: "mainThreadCompatibility",
      reason: "explicitCompatibilityDeployment",
      compatibilityRecovery: null,
    });
  } finally {
    await cleanup();
  }
});

test("explainCreateSignalsConstruction surfaces the selected worker-first callable lane when a worker runtime is available", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { explainCreateSignalsConstruction, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const explanation = explainCreateSignalsConstruction();
    assert.equal(explanation.requestedDeployment, "workerFirst");
    assert.equal(explanation.selectedFamily, "workerFirst");
    assert.equal(explanation.selectedDeployment, "workerFirst");
    assert.equal(explanation.reason, "workerFirstImportedGraphCallableSurface");
    assert.equal(explanation.compatibilityRecovery, null);
  } finally {
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
