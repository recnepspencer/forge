import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("default worker-first root admits standalone and active-import async computed/output callback authoring with chained authored-derived reads", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });

  const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const count = compatibilitySignals.spec.input("count", 2);
  const doubled = compatibilitySignals.spec.output("doubled", {
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
  const graph = compatibilitySignals.graph("workerFirstAsyncRecipe", {
    inputs: { count },
    outputs: { doubled },
  });

  try {
    const workerSignals = await createSignals();
    const eagerCount = workerSignals.input(2, { debugName: "eagerCount" });
    const eagerDouble = workerSignals.computed(() => eagerCount() * 2);
    const eagerPanel = workerSignals.output(() => ({ total: eagerDouble() }));
    const eagerNamedPanel = workerSignals.scope("draft").outputCallback(
      "namedPanel",
      () => ({ total: eagerCount() }),
    );
    assert.equal(eagerDouble(), 4);
    assert.deepEqual(eagerPanel(), { total: 4 });
    assert.deepEqual(eagerNamedPanel(), { total: 2 });

    const preImportCount = await workerSignals.inputAsync(2);
    const preImportDouble = await workerSignals.computedAsync({
      reads: [preImportCount.id],
      expr: {
        kind: "sum",
        args: [
          { kind: "read", id: preImportCount.id },
          { kind: "read", id: preImportCount.id },
        ],
      },
      identity: { kind: "exact" },
    });
    const preImportTriple = await workerSignals.computedAsync(
      () => preImportCount() * 3,
    );
    const preImportPanel = await workerSignals.scope("draft").outputAsync(
      () => ({
        total: preImportDouble(),
        triple: preImportTriple(),
      }),
    );

    assert.equal(preImportDouble(), 4);
    assert.equal(preImportTriple(), 6);
    assert.deepEqual(preImportPanel(), { total: 4, triple: 6 });
    await preImportCount.set(5);
    assert.equal(preImportDouble(), 10);
    assert.equal(preImportTriple(), 15);
    assert.deepEqual(preImportPanel(), { total: 10, triple: 15 });

    const runSummary = await workerSignals.transactionAsync((tx) => {
      tx.set(eagerCount, 9);
      tx.set(preImportCount, 7);
    });
    assert.equal(runSummary.touchedNodes > 0, true);
    assert.equal(eagerDouble(), 18);
    assert.deepEqual(eagerPanel(), { total: 18 });
    assert.deepEqual(eagerNamedPanel(), { total: 9 });
    assert.equal(preImportDouble(), 14);
    assert.equal(preImportTriple(), 21);
    assert.deepEqual(preImportPanel(), { total: 14, triple: 21 });

    const importedGraph = workerSignals.importGraph(
      graph.exportDefinition(),
      graph.exportSnapshot(),
    );
    await importedGraph.ready();

    assert.throws(() => preImportDouble(), /replaced the worker-owned runtime/);
    assert.throws(() => eagerDouble(), /replaced the worker-owned runtime/);
    assert.throws(() => eagerPanel(), /replaced the worker-owned runtime/);
    assert.throws(() => preImportTriple(), /replaced the worker-owned runtime/);
    assert.throws(() => preImportPanel(), /replaced the worker-owned runtime/);

    const authoredCount = await workerSignals.inputAsync(3, { debugName: "authoredCount" });
    const postImportTotal = await workerSignals.computedAsync(
      () => authoredCount() + importedGraph.input("count")(),
    );
    const postImportPanel = await workerSignals.scope("dashboard").outputAsync(
      () => ({ total: postImportTotal() }),
    );

    assert.equal(postImportTotal(), 5);
    assert.deepEqual(postImportPanel(), { total: 5 });

    const runSummary2 = await workerSignals.transactionAsync((tx) => {
      tx.set(authoredCount, 7);
      tx.set(importedGraph.input("count"), 11);
    });
    assert.equal(runSummary2.touchedNodes > 0, true);
    assert.equal(postImportTotal(), 18);
    assert.deepEqual(postImportPanel(), { total: 18 });

    await assert.rejects(
      workerSignals.computedAsync(() => {
        authoredCount.set(1);
        return 1;
      }),
      /cannot mutate signals or transactions/,
    );

    const replacement = compatibilitySignals.graph("workerFirstAsyncRecipeReplacement", {
      inputs: { replacement: compatibilitySignals.spec.input("replacement", 1) },
      outputs: {
        replacement: compatibilitySignals.spec.output("replacement.output", {
          reads: ["replacement"],
          expr: { kind: "read", id: "replacement" },
          identity: { kind: "exact" },
        }),
      },
    });
    const replacementImport = workerSignals.importGraph(
      replacement.exportDefinition(),
      replacement.exportSnapshot(),
    );
    await replacementImport.ready();

    assert.throws(() => postImportTotal(), /replaced the worker-owned runtime/);
    assert.throws(() => postImportPanel(), /replaced the worker-owned runtime/);

    workerSignals.free();
  } finally {
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
