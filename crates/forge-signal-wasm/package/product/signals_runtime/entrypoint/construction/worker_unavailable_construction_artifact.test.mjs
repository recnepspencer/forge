import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("createSignals rejects with a typed worker-unavailable construction artifact when no Worker constructor is available", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = undefined;
  const { createSignals, explainCreateSignalsConstruction, cleanup } = await loadSignalsModule();
  try {
    await assert.rejects(
      () => createSignals(),
      (error) => {
        assert.equal(error?.artifactFamily, "workerUnavailableConstruction");
        assert.equal(error?.requestedDeployment, "workerFirst");
        assert.equal(error?.reason, "workerConstructorUnavailable");
        assert.deepEqual(error?.compatibilityRecovery, {
          deployment: "mainThreadCompatibility",
          message: "Retry with deployment: \"mainThreadCompatibility\" to construct the explicit main-thread runtime lane.",
        });
        assert.deepEqual(error?.explanation, explainCreateSignalsConstruction());
        return true;
      },
    );
  } finally {
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
