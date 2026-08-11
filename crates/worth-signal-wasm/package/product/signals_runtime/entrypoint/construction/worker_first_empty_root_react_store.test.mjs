import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadStoreModule } from "../../../host_capabilities_certification/module_loading/load_store_module.mjs";
import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("worker-first empty root allows createReactSignalsStore without an imported graph", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup: cleanupSignals } = await loadSignalsModule({
    rawSurface: "real",
  });
  const { createReactSignalsStore, cleanup: cleanupStore } = await loadStoreModule();
  let signals = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    const diagnostics = signals.diagnostics();
    assert.equal(diagnostics.latestObservation(), null);
    assert.equal(diagnostics.latestFlow(), null);
    assert.equal(diagnostics.performanceSummary().deliveredObservationCount, 0);

    const store = createReactSignalsStore(signals);
    assert.equal(store.signals, signals);
    assert.equal(store.getDiagnosticsSnapshot().latestObservation, null);
    assert.equal(store.getDiagnosticsSnapshot().latestFlow, null);
    assert.equal(
      store.getDiagnosticsSnapshot().performanceSummary.deliveredObservationCount,
      0,
    );

    const unsubscribe = store.subscribeDiagnostics(() => {});
    unsubscribe();
    store.dispose();
  } finally {
    if (signals) {
      await signals.terminate();
    }
    await cleanupStore();
    await cleanupSignals();
    globalThis.Worker = previousWorker;
  }
});

test("worker-first empty root rejects diagnostics.why for unknown ids without requiring import context", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  let signals = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    assert.throws(
      () => signals.diagnostics().why("missing-signal-id"),
      /known authored signal id/u,
    );
  } finally {
    if (signals) {
      await signals.terminate();
    }
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
