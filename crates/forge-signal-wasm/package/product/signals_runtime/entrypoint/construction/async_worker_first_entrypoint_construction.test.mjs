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
    const count = signals.input(1, { debugName: "count" });
    const declarativeDouble = signals.computed({
      reads: [count.id],
      expr: {
        kind: "sum",
        args: [
          { kind: "read", id: count.id },
          { kind: "read", id: count.id },
        ],
      },
      identity: { kind: "exact" },
    });
    const doubled = signals.computed(() => count() * 2);
    const panel = signals.output(() => ({ count: doubled() }));
    const declarativePanel = signals.output({
      reads: [declarativeDouble.id],
      expr: {
        kind: "object",
        fields: [["count", { kind: "read", id: declarativeDouble.id }]],
      },
      identity: { kind: "exact" },
    });
    const explicitPanel = signals.outputCallback("panel", () => ({ count: count() }));

    assert.equal(count(), 1);
    assert.equal(declarativeDouble(), 2);
    assert.equal(doubled(), 2);
    assert.deepEqual(panel(), { count: 2 });
    assert.deepEqual(declarativePanel(), { count: 2 });
    assert.deepEqual(explicitPanel(), { count: 1 });
    await signals.transaction((tx) => {
      tx.set(count, 4);
    });
    assert.equal(declarativeDouble(), 8);
    assert.deepEqual(declarativePanel(), { count: 8 });
    signals.free();
  } finally {
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("createSignals keeps the default worker-first root explicit about the remaining sync-only compatibility lanes", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals();
    const count = signals.input(1, { debugName: "count" });
    const gate = signals.input(false, { debugName: "gate" });
    const gatedValue = signals.input(10, { debugName: "gatedValue" });
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
    const gatedComputed = signals.computed({
      reads: [count.id],
      when: {
        expr: {
          kind: "gt",
          left: { kind: "read", id: count.id },
          right: { kind: "value", value: 2 },
        },
      },
      expr: {
        kind: "sum",
        args: [
          { kind: "read", id: count.id },
          { kind: "value", value: 10 },
        ],
      },
      identity: { kind: "exact" },
    });
    const gatedOutput = signals.output({
      reads: [count.id],
      when: {
        expr: {
          kind: "gt",
          left: { kind: "read", id: count.id },
          right: { kind: "value", value: 2 },
        },
      },
      expr: {
        kind: "object",
        fields: [["count", { kind: "read", id: count.id }]],
      },
      identity: { kind: "exact" },
    });
    assert.equal(gatedComputed(), 11);
    assert.deepEqual(gatedOutput(), { count: 1 });
    await signals.transaction((tx) => {
      tx.set(count, 4);
    });
    assert.equal(gatedComputed(), 14);
    assert.deepEqual(gatedOutput(), { count: 4 });
    await signals.transaction((tx) => {
      tx.set(count, 1);
    });
    assert.equal(gatedComputed(), 14);
    assert.deepEqual(gatedOutput(), { count: 4 });
    const suppressedUntilGate = signals.output({
      reads: [gate.id, gatedValue.id],
      when: { expr: { kind: "read", id: gate.id } },
      expr: {
        kind: "object",
        fields: [["value", { kind: "read", id: gatedValue.id }]],
      },
      identity: { kind: "exact" },
    });
    assert.deepEqual(suppressedUntilGate(), { value: 10 });
    await signals.transaction((tx) => {
      tx.set(gatedValue, 12);
    });
    assert.deepEqual(suppressedUntilGate(), { value: 10 });
    await signals.transaction((tx) => {
      tx.set(gate, true);
    });
    assert.deepEqual(suppressedUntilGate(), { value: 12 });
    assert.throws(
      () => signals.computed({ when: null, expr: { kind: "value", value: 1 } }),
      /requires spec.when as a condition object/,
    );
    assert.throws(
      () => signals.output({ expr: { kind: "read", id: "missing" }, reads: ["missing"] }),
      /currently available/,
    );
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

test("createSignals admits worker-first persistence host-capability plans and replays them across runtime replacement", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const {
    createSignals,
    hostCapabilityPlan,
    persistenceCapability,
    cleanup,
  } = await loadSignalsModule({ rawSurface: "real" });
  let sourceSignals = null;
  let workerSignals = null;
  let importedGraph = null;
  let replacedGraph = null;
  const persistenceSource = createMutablePersistenceSource({ revision: 1 });
  try {
    workerSignals = await createSignals({
      hostCapabilities: hostCapabilityPlan({
        persistence: persistenceCapability({
          source: persistenceSource,
        }),
      }),
    });
    assert.deepEqual(workerSignals.host.persistence.value(), { revision: 1 });
    assert.throws(() => {
      workerSignals.host.persistence.value().revision = 99;
    }, /read only|Cannot assign|object is not extensible/i);
    assert.deepEqual(workerSignals.host.persistence.value(), { revision: 1 });

    persistenceSource.set({ revision: 2 });
    const firstCommit = await workerSignals.host.persistence.commit();
    const noOpCommit = await workerSignals.host.persistence.commit();
    assert.equal(typeof firstCommit.touchedNodes, "number");
    assert.deepEqual(noOpCommit, { touchedNodes: 0, nodesRecomputed: 0 });
    assert.equal(workerSignals.diagnostics().latestHostCapabilityEvent()?.family, "persistence");
    assert.equal(
      workerSignals.diagnostics().hostCapabilityReport().totals.manualCommitCount,
      2,
    );
    assert.equal(
      workerSignals.diagnostics().hostCapabilityReport().totals.noOpManualCommitCount,
      1,
    );

    sourceSignals = await createSignals({ deployment: "mainThreadCompatibility" });
    const count = sourceSignals.input(3, { debugName: "count" });
    const graph = sourceSignals.graph("workerFirstPersistenceImport", {
      inputs: { count },
      outputs: { count },
    });
    importedGraph = workerSignals.importGraph(graph.exportDefinition(), graph.exportSnapshot());
    await importedGraph.ready();

    persistenceSource.set({ revision: 5 });
    await workerSignals.host.persistence.commit();
    assert.deepEqual(workerSignals.host.persistence.value(), { revision: 5 });

    replacedGraph = workerSignals.importGraph(graph.exportDefinition(), graph.exportSnapshot());
    await replacedGraph.ready();

    persistenceSource.set({ revision: 6 });
    await workerSignals.host.persistence.commit();
    assert.deepEqual(workerSignals.host.persistence.value(), { revision: 6 });
  } finally {
    await replacedGraph?.terminate();
    await importedGraph?.terminate();
    workerSignals?.free();
    sourceSignals?.free();
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

function createMutablePersistenceSource(initialValue) {
  let currentValue = initialValue;
  return {
    current() {
      return currentValue;
    },
    set(nextValue) {
      currentValue = nextValue;
    },
  };
}
