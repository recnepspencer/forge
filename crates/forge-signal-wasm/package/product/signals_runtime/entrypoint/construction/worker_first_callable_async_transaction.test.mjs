import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("default worker-first root exposes async transaction and batch mutation over active imported-graph inputs", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });

  const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const wizard = compatibilitySignals.scope("wizard");
  const count = wizard.spec.input("count", 2);
  const item = wizard.spec.input("item", { label: "alpha", count: 1 });
  const doubled = wizard.spec.computed("doubled", {
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
  const graph = compatibilitySignals.graph("workerFirstAsyncTransaction", {
    inputs: { count, item },
    outputs: { doubled },
  });
  const compatibilityImportedSignals = await createSignals({
    deployment: "mainThreadCompatibility",
  });
  const definition = graph.exportDefinition();
  const snapshot = graph.exportSnapshot();
  const compatibilityImportedGraph = compatibilityImportedSignals.importGraph(definition, snapshot);
  await compatibilityImportedGraph.ready();

  try {
    const workerSignals = await createSignals();
    const importedGraph = workerSignals.importGraph(definition, snapshot);
    await importedGraph.ready();

    const rootCount = workerSignals.spec.input("wizard.count", 2);
    const rootItem = workerSignals.spec.input("wizard.item", { label: "alpha", count: 1 });

    const transactionSummary = await workerSignals.transactionAsync((tx) => {
      tx.set(rootCount, 7);
      tx.patch(rootItem, { label: "beta" });
    });
    await compatibilityImportedGraph.apply({
      writes: { count: 7 },
      patches: { item: { label: "beta" } },
    });

    assert.equal(transactionSummary.touchedNodes > 0, true);
    assert.equal(importedGraph.input("count")(), 7);
    assert.deepEqual(importedGraph.input("item")(), { label: "beta", count: 1 });
    assert.equal(workerSignals.read(importedGraph.output("doubled")), 14);
    assert.equal(
      workerSignals.read(importedGraph.output("doubled")),
      compatibilityImportedGraph.read().doubled,
    );

    const batchSummary = await workerSignals.batchAsync((tx) => {
      tx.set(importedGraph.input("count"), 9);
      tx.patch(rootItem, { count: 4 });
    });
    await compatibilityImportedGraph.apply({
      writes: { count: 9 },
      patches: { item: { count: 4 } },
    });

    assert.equal(batchSummary.touchedNodes > 0, true);
    assert.equal(importedGraph.input("count")(), 9);
    assert.deepEqual(importedGraph.input("item")(), { label: "beta", count: 4 });
    assert.deepEqual(importedGraph.read(), compatibilityImportedGraph.read());

    const foreignSignals = await createSignals({ deployment: "mainThreadCompatibility" });
    const foreignInput = foreignSignals.input(1);
    try {
      await assert.rejects(
        () =>
          workerSignals.transactionAsync((tx) => {
            tx.set(foreignInput, 3);
          }),
        /worker-first input handle/,
      );
    } finally {
      foreignSignals.free();
    }

    workerSignals.free();
  } finally {
    compatibilityImportedSignals.free();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
