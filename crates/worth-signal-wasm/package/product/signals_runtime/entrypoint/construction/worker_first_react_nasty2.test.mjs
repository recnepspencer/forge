import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadStoreModule } from "../../../host_capabilities_certification/module_loading/load_store_module.mjs";
import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

async function flush() {
  await Promise.resolve();
  await Promise.resolve();
  await new Promise((resolve) => setTimeout(resolve, 0));
}

test("NASTY2: importGraph from inside diagnostics listener must not deadlock the runtime", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  let signals = null;
  let compatibility = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    compatibility = await createSignals({ deployment: "mainThreadCompatibility" });
    const count = compatibility.input(2);
    const graph = compatibility.graph("nastyImportFromListener", {
      inputs: { count },
      outputs: {
        doubled: compatibility.computedSpec("nasty:import:listener:doubled", {
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

    let importPromise = null;
    const quantity = signals.input(1, { debugName: "nasty2.import.listener.q" });
    signals.diagnostics().subscribe(() => {
      if (importPromise === null) {
        importPromise = signals.importGraph(definition, snapshot).ready();
      }
    });

    await signals.transaction((tx) => tx.set(quantity, 2));
    assert.ok(importPromise, "listener should have started importGraph");
    await Promise.race([
      importPromise,
      new Promise((_, reject) => {
        setTimeout(() => reject(new Error("importGraph from diagnostics listener deadlocked")), 5000);
      }),
    ]);
    assert.equal(signals.read(graph.output("doubled").id), 4);
  } finally {
    if (signals) await signals.terminate().catch(() => {});
    if (compatibility) compatibility.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("NASTY2: React getDiagnosticsSnapshot identity must be stable between notifies", async () => {
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
    store.subscribeDiagnostics(() => {});
    const first = store.getDiagnosticsSnapshot();
    const second = store.getDiagnosticsSnapshot();
    assert.equal(
      first,
      second,
      "useSyncExternalStore requires referential stability between notifications",
    );
    const quantity = signals.input(1, { debugName: "nasty2.identity.q" });
    await signals.transaction((tx) => tx.set(quantity, 2));
    await flush();
    const third = store.getDiagnosticsSnapshot();
    assert.notEqual(third, first, "after notify, snapshot identity should advance");
    store.dispose();
  } finally {
    if (signals) await signals.terminate();
    await cleanupStore();
    await cleanupSignals();
    globalThis.Worker = previousWorker;
  }
});

test("NASTY2: partial/fake signals object must not attach a half-alive React store", async () => {
  const { createReactSignalsStore, cleanup } = await loadStoreModule();
  try {
    assert.throws(
      () => createReactSignalsStore({
        read() { return 1; },
      }),
      /diagnostics|is not a function|undefined/iu,
    );
    assert.throws(
      () => createReactSignalsStore({
        read() { return 1; },
        diagnostics() {
          return {
            latestObservation() { return null; },
            latestFlow() { return null; },
          };
        },
      }),
      /performanceSummary|is not a function|undefined/iu,
    );
  } finally {
    await cleanup();
  }
});

test("NASTY2: replace import while React is watching imported output", async () => {
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
    const count = compatibility.input(3);
    const graph = compatibility.graph("nastyReplaceWatch", {
      inputs: { count },
      outputs: {
        doubled: compatibility.computedSpec("nasty:replace:watch:doubled", {
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
    const first = worker.importGraph(definition, snapshot);
    await first.ready();
    const notices = [];
    const unsub = store.subscribeSignal(outputId, () => {
      notices.push(store.getSignalSnapshot(outputId));
    });
    assert.equal(store.getSignalSnapshot(outputId), 6);

    // Second import supersedes the first while React still watches the old id.
    const second = worker.importGraph(definition, snapshot);
    await second.ready();
    await flush();
    await second.writeInput("count", 10);
    await flush();

    // After supersession the watch must track the new import (not stay stuck at 6).
    assert.equal(store.getSignalSnapshot(outputId), 20);
    assert.equal(notices.includes(20), true);

    unsub();
    store.dispose();
  } finally {
    if (worker) await worker.terminate();
    if (compatibility) compatibility.free();
    await cleanupStore();
    await cleanupSignals();
    globalThis.Worker = previousWorker;
  }
});
