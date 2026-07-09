import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

function comparableSurface(surface) {
  return JSON.parse(JSON.stringify(surface));
}

function comparableSnapshotSources(snapshot) {
  return snapshot.snapshotEnvelope.state.sources.map((source) => ({
    id: source.id,
    value: source.value,
  }));
}

test("worker-first imported graph preserves committed public graph truth after worker hydration from exported input state", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const mod = await loadSignalsModule({ rawSurface: "real" });
  const { createSignals, cleanup, importProductModule } = mod;
  const { createWorkerFirstImportedGraphSession } = await importProductModule(
    "entrypoint/worker_first_imported_graph.js",
  );

  const sourceSignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const count = sourceSignals.input(2, { debugName: "count" });
  const item = sourceSignals.input({ label: "alpha" }, { debugName: "item" });
  const publishedGraph = sourceSignals.graph("importedWorker", {
    inputs: { count, item },
    outputs: { count, item },
  });
  count.set(6);
  item.patch({ label: "beta" });
  const exportedDefinition = publishedGraph.exportDefinition();
  const exportedSnapshot = publishedGraph.exportSnapshot();

  const restoredSignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const compatibilityImportedGraph = restoredSignals.importGraph(
    exportedDefinition,
    exportedSnapshot,
  );
  const workerImportedGraph = await createWorkerFirstImportedGraphSession({
    definition: exportedDefinition,
    snapshot: exportedSnapshot,
  });

  try {
    const workerDiagnostics = await workerImportedGraph.inspectDiagnostics();
    const compatibilityDiagnostics = compatibilityImportedGraph.inspectDiagnostics();
    const workerItemInput = workerImportedGraph.input("item").get();
    assert.equal(Object.isFrozen(workerItemInput), true);
    assert.throws(() => {
      workerItemInput.label = "mutated";
    }, /Cannot assign/);

    assert.deepEqual(workerImportedGraph.readInputs(), compatibilityImportedGraph.readInputs());
    assert.deepEqual(workerImportedGraph.read(), compatibilityImportedGraph.read());
    assert.deepEqual(workerImportedGraph.contract(), compatibilityImportedGraph.contract());
    assert.deepEqual(workerImportedGraph.contractHistory(), exportedSnapshot.contractHistory);
    assert.deepEqual(workerImportedGraph.importPosture(), {
      ...exportedSnapshot.importPosture,
      graphId: exportedSnapshot.id,
      hydrate: "Applied",
      hydrateReason:
        "worker-first imported graph hydrated tracked public inputs from exported snapshot state",
    });
    assert.deepEqual(
      comparableSurface({
        graph: workerDiagnostics.graph,
        contract: workerDiagnostics.contract,
        dependencies: { ...workerDiagnostics.dependencies },
        inputDescriptors: workerDiagnostics.inputDescriptors,
        descriptors: workerDiagnostics.descriptors,
        inputVersions: workerDiagnostics.inputVersions.map((entry) => ({
          id: entry.id,
          version: entry.version,
        })),
        outputVersions: workerDiagnostics.outputVersions.map((entry) => ({
          id: entry.id,
          version: entry.version,
        })),
      }),
      comparableSurface({
        graph: compatibilityDiagnostics.graph,
        contract: compatibilityDiagnostics.contract,
        dependencies: { ...compatibilityDiagnostics.dependencies },
        inputDescriptors: compatibilityDiagnostics.inputDescriptors,
        descriptors: compatibilityDiagnostics.descriptors,
        inputVersions: compatibilityDiagnostics.inputVersions.map((entry) => ({
          id: entry.id,
          version: entry.version,
        })),
        outputVersions: compatibilityDiagnostics.outputVersions.map((entry) => ({
          id: entry.id,
          version: entry.version,
        })),
      }),
    );
    const workerHistory = await workerImportedGraph.inspectHistory();
    assert.equal(Array.isArray(workerHistory.recentHistory), true);
    assert.equal(workerHistory.recentHistory.length > 0, true);
    assert.equal(workerHistory.input("count").replay.frames.length > 0, true);
    assert.equal(workerHistory.output("count").descriptor.outputName, "count");
    assert.deepEqual(
      workerImportedGraph.exportCompatibilityDefinition(),
      compatibilityImportedGraph.exportCompatibilityDefinition(),
    );
    assert.deepEqual(
      workerImportedGraph.exportDefinition(),
      compatibilityImportedGraph.exportDefinition(),
    );
    const workerSnapshot = workerImportedGraph.exportSnapshot();
    assert.equal(workerSnapshot.id, exportedSnapshot.id);
    assert.deepEqual(workerSnapshot.definition, exportedSnapshot.definition);
    assert.equal(workerSnapshot.restoreMode, exportedSnapshot.restoreMode);
    assert.equal(
      typeof workerSnapshot.runtimeEnvelope.runtimeEnvelopePortableWire,
      "string",
    );
    assert.equal(
      Array.isArray(workerSnapshot.snapshotEnvelope.state.sources),
      true,
    );
  } finally {
    await workerImportedGraph.terminate();
    sourceSignals.free();
    restoredSignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("worker-first imported graph rejects malformed import pairs before worker truth is claimed", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const mod = await loadSignalsModule({ rawSurface: "real" });
  const { createSignals, cleanup, importProductModule } = mod;
  const { createWorkerFirstImportedGraphSession } = await importProductModule(
    "entrypoint/worker_first_imported_graph.js",
  );
  const sourceSignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const count = sourceSignals.input(1, { debugName: "count" });
  const graph = sourceSignals.graph("badImport", {
    inputs: { count },
    outputs: { count },
  });

  try {
    await assert.rejects(
      () =>
        createWorkerFirstImportedGraphSession({
          definition: graph.exportDefinition(),
          snapshot: { ...graph.exportSnapshot(), id: "other" },
        }),
      /requires matching graph ids/,
    );
  } finally {
    sourceSignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("worker-first imported graph mutates imported public inputs with parity against compatibility imported graphs", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const mod = await loadSignalsModule({ rawSurface: "real" });
  const { createSignals, cleanup, importProductModule } = mod;
  const { createWorkerFirstImportedGraphSession } = await importProductModule(
    "entrypoint/worker_first_imported_graph.js",
  );

  const sourceSignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const item = sourceSignals.input({ label: "alpha", count: 1 }, { debugName: "item" });
  const graph = sourceSignals.graph("mutatingImportedWorker", {
    inputs: { item },
    outputs: { item },
  });
  const definition = graph.exportDefinition();
  const snapshot = graph.exportSnapshot();
  const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const compatibilityImportedGraph = compatibilitySignals.importGraph(definition, snapshot);
  const workerImportedGraph = await createWorkerFirstImportedGraphSession({
    definition,
    snapshot,
  });

  try {
    assert.deepEqual(
      compatibilityImportedGraph.operationalContract(),
      workerImportedGraph.operationalContract(),
    );
    await compatibilityImportedGraph.input("item").patch({ label: "beta" });
    await workerImportedGraph.input("item").patch({ label: "beta" });
    await compatibilityImportedGraph.inputs.item.set({ label: "gamma", count: 3 });
    await workerImportedGraph.inputs.item.set({ label: "gamma", count: 3 });
    await compatibilityImportedGraph.inputs.item.assign({ count: 9 });
    await workerImportedGraph.inputs.item.assign({ count: 9 });
    await compatibilityImportedGraph.input("item").reset();
    await workerImportedGraph.input("item").reset();
    await compatibilityImportedGraph.apply({ writes: { item: { label: "delta", count: 4 } } });
    await workerImportedGraph.apply({ writes: { item: { label: "delta", count: 4 } } });

    assert.deepEqual(workerImportedGraph.readInputs(), compatibilityImportedGraph.readInputs());
    assert.deepEqual(workerImportedGraph.read(), compatibilityImportedGraph.read());
    assert.deepEqual(
      comparableSnapshotSources(workerImportedGraph.exportSnapshot()),
      comparableSnapshotSources(compatibilityImportedGraph.exportSnapshot()),
    );
  } finally {
    await workerImportedGraph.terminate();
    sourceSignals.free();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
