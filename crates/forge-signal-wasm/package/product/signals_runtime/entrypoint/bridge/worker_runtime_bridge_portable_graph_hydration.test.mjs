import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("createWorkerRuntimeBridge publishes a portable public graph and hydrates committed public input truth without inventing main-thread runtime truth", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const mod = await loadSignalsModule({ rawSurface: "real" });
  const { createSignals, createWorkerRuntimeBridge, cleanup } = mod;
  const targetBridge = createWorkerRuntimeBridge();

  const compatibilitySignals = await createSignals({
    deployment: "mainThreadCompatibility",
  });
  const compatibilityGraph = compatibilitySignals.graph("portableSnapshot", (graph) => {
    const scope = graph.scope("worker");
    const count = scope.input(3, { id: "count" });
    const doubled = scope.computedSpec("doubled", {
      reads: [count.id],
      expr: {
        kind: "sum",
        args: [{ kind: "read", id: count.id }, { kind: "read", id: count.id }],
      },
      identity: { kind: "exact" },
    });
    return graph.expose({ inputs: { count }, outputs: { doubled } });
  });

  try {
    const exportedDefinition = compatibilityGraph.exportDefinition();
    compatibilityGraph.writeInput("count", 8);
    await targetBridge.publishPortableGraph({
      ...exportedDefinition.compatibility.definitions,
      outputIds: compatibilityGraph.descriptors().map((entry) => entry.publishedId),
    });
    await targetBridge.applyTransaction([
      { kind: "set", id: exportedDefinition.inputDescriptors[0].sourceId, value: 8 },
    ]);
    assert.deepEqual(
      (await targetBridge.readSignals({ signalIds: ["portableSnapshot.worker.count"] })).signals[0]
        .value,
      compatibilityGraph.readInputs().count,
    );
    assert.deepEqual(
      (await targetBridge.deliverOutputs({ outputIds: ["portableSnapshot.doubled"] })).outputs[0]
        .value,
      compatibilityGraph.read().doubled,
    );
  } finally {
    await targetBridge.terminate();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("createWorkerRuntimeBridge publishPortableGraph rejects invalid published output claims before worker truth is claimed", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, createWorkerRuntimeBridge, cleanup } = await loadSignalsModule({
    rawSurface: "real",
  });
  const targetBridge = createWorkerRuntimeBridge();
  const compatibilitySignals = await createSignals({
    deployment: "mainThreadCompatibility",
  });
  const count = compatibilitySignals.input(2, { debugName: "count" });
  const graph = compatibilitySignals.graph("portableDenial", {
    inputs: { count },
    outputs: { count },
  });

  try {
    await assert.rejects(
      () =>
        targetBridge.publishPortableGraph({
          ...graph.exportDefinition().compatibility.definitions,
          outputIds: ["portableDenial.missing"],
        }),
      /published recipe/,
    );
  } finally {
    await targetBridge.terminate();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
