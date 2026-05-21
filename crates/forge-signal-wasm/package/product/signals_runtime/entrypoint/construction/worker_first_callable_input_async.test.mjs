import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("default worker-first root admits explicit async input authoring and mixed async mutation with active imported graphs", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });

  const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const wizard = compatibilitySignals.scope("wizard");
  const importedCount = wizard.spec.input("count", 2);
  const importedDoubled = wizard.spec.output("doubled", {
    reads: [importedCount.id],
    expr: {
      kind: "sum",
      args: [
        { kind: "read", id: importedCount.id },
        { kind: "read", id: importedCount.id },
      ],
    },
    identity: { kind: "exact" },
  });
  const graph = compatibilitySignals.graph("workerFirstAsyncInput", {
    inputs: { count: importedCount },
    outputs: { doubled: importedDoubled },
  });
  const definition = graph.exportDefinition();
  const snapshot = graph.exportSnapshot();

  try {
    const workerSignals = await createSignals();
    const preImportInput = await workerSignals.inputAsync(1, {
      debugName: "preImportInput",
    });
    assert.equal(preImportInput(), 1);

    const importedGraph = workerSignals.importGraph(definition, snapshot);
    await importedGraph.ready();
    assert.throws(
      () => preImportInput(),
      /replaced the worker-owned runtime/,
    );

    const authoredCount = await workerSignals.inputAsync(3, {
      debugName: "authoredCount",
    });
    const scopedFormState = await workerSignals.scope("wizard").inputAsync(
      { label: "alpha", done: false },
      { id: "formState" },
    );

    assert.equal(authoredCount(), 3);
    assert.deepEqual(workerSignals.read(authoredCount), 3);
    assert.equal(scopedFormState.id, "wizard.formState");
    assert.deepEqual(scopedFormState(), { label: "alpha", done: false });

    const publicInput = workerSignals.publicInput(authoredCount, {
      authority: "writable",
      requiredness: "optional",
    });
    const controller = workerSignals.controller({
      inputs: {
        authoredCount: publicInput,
      },
      outputs: {},
      internal: {},
    });
    assert.equal(controller.inputs.authoredCount.handle.id, authoredCount.id);

    await authoredCount.set(4);
    await scopedFormState.assign({ done: true });
    assert.equal(workerSignals.read(authoredCount), 4);
    assert.deepEqual(scopedFormState(), { label: "alpha", done: true });

    const runSummary = await workerSignals.transactionAsync((tx) => {
      tx.set(authoredCount, 8);
      tx.patch(scopedFormState, { label: "beta" });
      tx.set(importedGraph.input("count"), 9);
    });

    assert.equal(runSummary.touchedNodes > 0, true);
    assert.equal(workerSignals.read(authoredCount), 8);
    assert.deepEqual(scopedFormState(), { label: "beta", done: true });
    assert.equal(importedGraph.input("count")(), 9);
    assert.equal(workerSignals.read(importedGraph.output("doubled")), 18);

    workerSignals.free();
  } finally {
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
