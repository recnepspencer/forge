import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("createSignals constructs the default worker-first callable root when a worker runtime is available", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals();
    assert.equal(typeof signals.importGraph, "function");
    assert.equal(typeof signals.free, "function");
    assert.throws(
      () => signals.input(1),
      (error) =>
        error?.name === "WorkerFirstCallableSurfaceUnavailable"
        && error?.compatibilityRecovery?.deployment === "mainThreadCompatibility",
    );
    signals.free();
  } finally {
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("createSignals keeps the default worker-first root explicit about compatibility-only root authoring lanes", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals();
    assert.throws(
      () => signals.graph("g", { outputs: { missing: {} } }),
      /active imported graph/,
    );
    assert.equal(typeof signals.controller, "function");
    assert.equal(typeof signals.publicInput, "function");
    assert.equal(typeof signals.scope, "function");
    assert.equal(typeof signals.graph, "function");
    assert.equal(typeof signals.diagnostics, "function");
    assert.equal(typeof signals.history, "function");
    assert.equal(typeof signals.adapters, "function");
    signals.free();
  } finally {
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("createSignals admits supported worker-first host-capability plans, keeps signals.host live, and preserves ordinary imported graph hydration", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const {
    createSignals,
    hostCapabilityPlan,
    onlineCapability,
    cleanup,
  } = await loadSignalsModule({ rawSurface: "real" });
  let sourceSignals = null;
  let workerSignals = null;
  let importedGraph = null;
  try {
    sourceSignals = await createSignals({ deployment: "mainThreadCompatibility" });
    const onlineSource = createSubscribableSource("online");
    const count = sourceSignals.input(4, { debugName: "count" });
    const ordinaryGraph = sourceSignals.graph("workerFirstHostCompatibleImport", {
      inputs: { count },
      outputs: { count },
    });
    count.set(9);
    const definition = ordinaryGraph.exportDefinition();
    const snapshot = ordinaryGraph.exportSnapshot();

    workerSignals = await createSignals({
      hostCapabilities: hostCapabilityPlan({
        online: onlineCapability({
          source: onlineSource.source,
        }),
      }),
    });
    assert.equal(workerSignals.host.online.state(), "online");
    onlineSource.set("offline");
    await waitForValue(() => workerSignals.host.online.state(), "offline");
    assert.equal(
      workerSignals.diagnostics().latestHostCapabilityEvent()?.family,
      "online",
    );
    assert.deepEqual(
      workerSignals.diagnostics().recentHostCapabilityEvents().map((event) => event.kind),
      ["InvalidationFlushed"],
    );
    assert.equal(
      workerSignals.diagnostics().hostCapabilityReport().totals.registrationCount,
      1,
    );
    assert.equal(
      workerSignals.diagnostics().hostCapabilityReport().totals.invalidationCount,
      1,
    );
    assert.equal(
      workerSignals.diagnostics().hostCapabilityReport().families[0].family,
      "online",
    );

    importedGraph = workerSignals.importGraph(definition, snapshot);
    await importedGraph.ready();
    assert.equal(importedGraph.readInputs().count, 9);
    assert.equal(importedGraph.read().count, 9);
    assert.equal(workerSignals.read(importedGraph.output("count")), 9);
  } finally {
    await importedGraph?.terminate();
    workerSignals?.free();
    sourceSignals?.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("createSignals still rejects worker-first persistence host-capability plans", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const {
    createSignals,
    hostCapabilityPlan,
    persistenceCapability,
    cleanup,
  } = await loadSignalsModule({ rawSurface: "real" });
  try {
    await assert.rejects(
      () => createSignals({
        hostCapabilities: hostCapabilityPlan({
          persistence: persistenceCapability({
            source: {
              current() {
                return { revision: 1 };
              },
            },
          }),
        }),
      }),
      (error) =>
        error?.name === "signalsConstructionDenied"
        && error?.reason === "workerFirstPersistenceHostCapabilityNotImplemented",
    );
  } finally {
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

function createSubscribableSource(initialValue) {
  let currentValue = initialValue;
  const listeners = new Set();

  return {
    source: {
      current() {
        return currentValue;
      },
      subscribe(listener) {
        listeners.add(listener);
        return () => listeners.delete(listener);
      },
    },
    set(nextValue) {
      currentValue = nextValue;
      for (const listener of listeners) {
        listener();
      }
    },
  };
}

async function waitForValue(read, expected) {
  for (let attempt = 0; attempt < 100; attempt += 1) {
    if (read() === expected) {
      return;
    }
    await new Promise((resolve) => setTimeout(resolve, 10));
  }
  assert.equal(read(), expected);
}
