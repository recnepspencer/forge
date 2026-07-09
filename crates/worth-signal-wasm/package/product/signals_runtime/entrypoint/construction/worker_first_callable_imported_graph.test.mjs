import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import {
  materializeGraphDiagnosticsSurface,
  materializeGraphHistorySurface,
} from "../../runtime_fixture/surface_materialization.mjs";

function comparableDiagnosticsSurface(surface) {
  const materialized = materializeGraphDiagnosticsSurface(surface);
  return {
    graph: materialized.graph,
    contract: materialized.contract,
    dependencies: materialized.dependencies,
    inputDescriptors: materialized.inputDescriptors,
    descriptors: materialized.descriptors,
    inputVersionIds: materialized.inputVersions.map((entry) => entry?.id ?? null),
    outputVersionIds: materialized.outputVersions.map((entry) => entry?.id ?? null),
    inputNames: Object.keys(materialized.inputs),
    outputNames: Object.keys(materialized.outputs),
    contractSummary: surface.contractSummary(),
  };
}

function comparableHistorySurface(surface) {
  const materialized = materializeGraphHistorySurface(surface);
  return {
    graph: materialized.graph,
    contract: materialized.contract,
    dependencies: materialized.dependencies,
    inputDescriptors: materialized.inputDescriptors,
    descriptors: materialized.descriptors,
    inputNames: Object.keys(materialized.inputs),
    outputNames: Object.keys(materialized.outputs),
    contractSummary: surface.contractSummary(),
  };
}

function comparableSnapshotSources(snapshot) {
  return snapshot.snapshotEnvelope.state.sources.map((source) => ({
    id: source.id,
    value: source.value,
  }));
}

test("default worker-first createSignals admits imported graphs through explicit readiness and converges to compatibility truth", async () => {
  const previousWorker = globalThis.Worker;
  let workerConstructionCount = 0;
  globalThis.Worker = class CountingWorker extends NodeWorker {
    constructor(url, options) {
      workerConstructionCount += 1;
      super(url, options);
    }
  };
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });

  const sourceSignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const count = sourceSignals.input(2, { debugName: "count" });
  const item = sourceSignals.input({ label: "alpha" }, { debugName: "item" });
  const publishedGraph = sourceSignals.graph("workerFirstRootImport", {
    inputs: { count, item },
    outputs: { count, item },
  });
  count.set(8);
  item.patch({ label: "beta" });

  const definition = publishedGraph.exportDefinition();
  const snapshot = publishedGraph.exportSnapshot();
  const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const compatibilityImportedGraph = compatibilitySignals.importGraph(definition, snapshot);

  try {
    const workerSignals = await createSignals();
    assert.equal(workerConstructionCount, 1);
    const importedGraph = workerSignals.importGraph(definition, snapshot);
    assert.equal(typeof importedGraph.ready, "function");
    assert.equal(importedGraph.importPosture().hydrate, "Deferred");
    assert.throws(
      () => importedGraph.read(),
      /requires await importedGraph\.ready\(\)/,
    );
    await importedGraph.ready();
    assert.equal(importedGraph.importPosture().hydrate, "Applied");
    assert.deepEqual(importedGraph.readInputs(), compatibilityImportedGraph.readInputs());
    assert.deepEqual(importedGraph.read(), compatibilityImportedGraph.read());
    assert.deepEqual(importedGraph.contract(), compatibilityImportedGraph.contract());
    const diagnosticsSurface = importedGraph.inspectDiagnostics();
    const historySurface = importedGraph.inspectHistory();
    assert.equal(typeof diagnosticsSurface.then, "undefined");
    assert.equal(typeof historySurface.then, "undefined");
    assert.deepEqual(
      comparableDiagnosticsSurface(diagnosticsSurface),
      comparableDiagnosticsSurface(compatibilityImportedGraph.inspectDiagnostics()),
    );
    assert.deepEqual(
      comparableHistorySurface(historySurface),
      comparableHistorySurface(compatibilityImportedGraph.inspectHistory()),
    );

    const laterSignals = await createSignals({ deployment: "mainThreadCompatibility" });
    const otherCount = laterSignals.input(3, { debugName: "otherCount" });
    const otherGraph = laterSignals.graph("workerFirstRootImportOther", {
      inputs: { otherCount },
      outputs: { otherCount },
    });
    otherCount.set(15);
    const newerImportedGraph = workerSignals.importGraph(
      otherGraph.exportDefinition(),
      otherGraph.exportSnapshot(),
    );
    assert.equal(workerConstructionCount, 1);
    await newerImportedGraph.ready();
    assert.equal(newerImportedGraph.read().otherCount, 15);
    assert.throws(
      () => importedGraph.read(),
      /superseded by a newer root importGraph\(\) call/,
    );
    laterSignals.free();
    await newerImportedGraph.terminate();
    await importedGraph.terminate();
    workerSignals.free();
  } finally {
    sourceSignals.free();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("default worker-first imported graphs mutate through the shared root-owned worker runtime", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });

  const sourceSignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const item = sourceSignals.input({ label: "alpha", count: 1 }, { debugName: "item" });
  const graph = sourceSignals.graph("workerFirstRootImportMutations", {
    inputs: { item },
    outputs: { item },
  });
  const definition = graph.exportDefinition();
  const snapshot = graph.exportSnapshot();

  const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const compatibilityImportedGraph = compatibilitySignals.importGraph(definition, snapshot);

  try {
    const workerSignals = await createSignals();
    const importedGraph = workerSignals.importGraph(definition, snapshot);
    await importedGraph.ready();
    await importedGraph.inputs.item.patch({ label: "beta" });
    await compatibilityImportedGraph.inputs.item.patch({ label: "beta" });
    await importedGraph.input("item").assign({ count: 2 });
    await compatibilityImportedGraph.input("item").assign({ count: 2 });
    await importedGraph.apply({ writes: { item: { label: "gamma", count: 3 } } });
    await compatibilityImportedGraph.apply({ writes: { item: { label: "gamma", count: 3 } } });

    assert.deepEqual(importedGraph.readInputs(), compatibilityImportedGraph.readInputs());
    assert.deepEqual(importedGraph.read(), compatibilityImportedGraph.read());
    assert.deepEqual(
      workerSignals.read(importedGraph.output("item")),
      compatibilityImportedGraph.output("item").get(),
    );
    assert.deepEqual(
      comparableSnapshotSources(importedGraph.exportSnapshot()),
      comparableSnapshotSources(compatibilityImportedGraph.exportSnapshot()),
    );

    await importedGraph.terminate();
    workerSignals.free();
  } finally {
    sourceSignals.free();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
