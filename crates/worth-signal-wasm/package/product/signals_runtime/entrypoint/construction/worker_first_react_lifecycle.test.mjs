import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadStoreModule } from "../../../host_capabilities_certification/module_loading/load_store_module.mjs";
import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

async function flushMicrotasks() {
  await Promise.resolve();
  await Promise.resolve();
  await new Promise((resolve) => setTimeout(resolve, 0));
}

test("worker-first React store: empty attach, standalone author, watch, diagnostics push", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup: cleanupSignals } = await loadSignalsModule({
    rawSurface: "real",
  });
  const { createReactSignalsStore, cleanup: cleanupStore } = await loadStoreModule();
  let signals = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    const store = createReactSignalsStore(signals);
    assert.equal(store.getDiagnosticsSnapshot().latestObservation, null);

    const diagnosticsNotifications = [];
    const unsubscribeDiagnostics = store.subscribeDiagnostics(() => {
      diagnosticsNotifications.push(store.getDiagnosticsSnapshot());
    });

    const quantity = signals.input(2, { debugName: "react.lifecycle.quantity" });
    const total = signals.computed(() => quantity() * 10, {
      debugName: "react.lifecycle.total",
    });

    assert.equal(store.getSignalSnapshot(total), 20);

    const signalNotifications = [];
    const unsubscribeSignal = store.subscribeSignal(total, () => {
      signalNotifications.push(store.getSignalSnapshot(total));
    });

    await signals.transaction((tx) => {
      tx.set(quantity, 5);
    });
    await flushMicrotasks();

    assert.equal(total(), 50);
    assert.equal(store.getSignalSnapshot(total), 50);
    assert.equal(signalNotifications.includes(50), true);

    assert.equal(diagnosticsNotifications.length > 0, true);
    const latestDiagnostics = store.getDiagnosticsSnapshot();
    assert.equal(
      typeof latestDiagnostics.performanceSummary?.deliveredObservationCount,
      "number",
    );

    const why = await signals.diagnostics().why(total.id);
    assert.equal(why.id, total.id);

    unsubscribeSignal();
    unsubscribeDiagnostics();
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

test("worker-first React store refreshes diagnostics across importGraph admission", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup: cleanupSignals } = await loadSignalsModule({
    rawSurface: "real",
  });
  const { createReactSignalsStore, cleanup: cleanupStore } = await loadStoreModule();
  let compatibility = null;
  let worker = null;
  try {
    compatibility = await createSignals({ deployment: "mainThreadCompatibility" });
    const count = compatibility.input(3, { debugName: "import.count" });
    const graph = compatibility.graph("workerFirstReactImport", {
      inputs: { count },
      outputs: {
        doubled: compatibility.computedSpec("react:import:doubled", {
          reads: [count.id],
          expr: {
            kind: "sum",
            args: [
              { kind: "read", id: count.id },
              { kind: "read", id: count.id },
            ],
          },
          identity: { kind: "exact" },
        }),
      },
    });
    const definition = graph.exportDefinition();
    const snapshot = graph.exportSnapshot();
    const outputId = graph.output("doubled").id;

    worker = await createSignals({ deployment: "workerFirst" });
    const store = createReactSignalsStore(worker);
    const diagnosticsNotifications = [];
    const unsubscribeDiagnostics = store.subscribeDiagnostics(() => {
      diagnosticsNotifications.push(store.getDiagnosticsSnapshot());
    });

    const imported = worker.importGraph(definition, snapshot);
    await imported.ready();
    await flushMicrotasks();

    assert.equal(diagnosticsNotifications.length > 0, true);
    assert.equal(store.getSignalSnapshot(outputId), 6);

    const notices = [];
    const unsubscribeSignal = store.subscribeSignal(outputId, () => {
      notices.push(store.getSignalSnapshot(outputId));
    });
    await imported.writeInput("count", 9);
    await flushMicrotasks();
    assert.equal(store.getSignalSnapshot(outputId), 18);
    assert.equal(notices.includes(18), true);

    unsubscribeSignal();
    unsubscribeDiagnostics();
    store.dispose();
  } finally {
    if (worker) {
      await worker.terminate();
    }
    if (compatibility) {
      compatibility.free();
    }
    await cleanupStore();
    await cleanupSignals();
    globalThis.Worker = previousWorker;
  }
});

test("worker-first empty root surface matrix: snapshot diagnostics ok; graph-scoped ops deny", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  let signals = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    assert.equal(signals.diagnostics().latestObservation(), null);
    assert.equal(signals.diagnostics().performanceSummary().deliveredObservationCount, 0);

    assert.throws(
      () => signals.adapters().exportRuntimeEnvelope(),
      /active imported graph/u,
    );
    assert.throws(
      () => signals.specialist().graphSummary(),
      /active imported graph/u,
    );
  } finally {
    if (signals) {
      await signals.terminate();
    }
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
