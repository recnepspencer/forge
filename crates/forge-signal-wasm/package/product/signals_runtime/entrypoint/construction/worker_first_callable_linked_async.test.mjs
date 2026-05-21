import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

const STALE_WORKER_FIRST_AUTHORITY = /replaced the worker-owned runtime|superseded by a newer root importGraph/;

test("default worker-first root admits standalone, scoped, and active-import linkedAsync authoring with runtime-replacement invalidation", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });

  const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const count = compatibilitySignals.spec.input("count", 2);
  const graph = compatibilitySignals.graph("workerFirstLinkedAsync", {
    inputs: { count },
    outputs: {
      count: compatibilitySignals.spec.output("count.output", {
        reads: [count.id],
        expr: { kind: "read", id: count.id },
        identity: { kind: "exact" },
      }),
    },
  });

  try {
    const workerSignals = await createSignals();
    const shippingOptions = await workerSignals.inputAsync(
      [
        { id: "ground", label: "Ground" },
        { id: "air", label: "Air" },
      ],
      { debugName: "shippingOptions" },
    );
    const firstOption = await workerSignals.linkedAsync(() => shippingOptions()[0], {
      debugName: "firstOption",
    });
    const preservedSelection = await workerSignals.scope("checkout").linkedAsync({
      source: () => shippingOptions(),
      computation: (options, previous) =>
        options.find((option) => option.id === previous?.value?.id) ?? options[0] ?? null,
      debugName: "preservedSelection",
    });

    assert.equal(firstOption.debugName, "firstOption");
    assert.equal(firstOption().id, "ground");
    assert.equal(preservedSelection().id, "ground");

    await preservedSelection.set({ id: "air", label: "Air" });
    await shippingOptions.set([
      { id: "ground", label: "Ground" },
      { id: "air", label: "Air" },
      { id: "sea", label: "Sea" },
    ]);
    await preservedSelection.relink();
    await preservedSelection.set({ id: "manual", label: "Manual" });
    await preservedSelection.reset();
    assert.equal(preservedSelection().id, "air");

    await firstOption.set({ id: "manual", label: "Manual" });
    await shippingOptions.set([
      { id: "sea", label: "Sea" },
      { id: "ground", label: "Ground" },
    ]);
    await firstOption.reset();
    assert.equal(firstOption().id, "sea");

    const importedGraph = workerSignals.importGraph(
      graph.exportDefinition(),
      graph.exportSnapshot(),
    );
    await importedGraph.ready();

    assert.throws(() => firstOption(), STALE_WORKER_FIRST_AUTHORITY);
    assert.throws(() => preservedSelection(), STALE_WORKER_FIRST_AUTHORITY);
    await assert.rejects(firstOption.relink(), STALE_WORKER_FIRST_AUTHORITY);
    await assert.rejects(preservedSelection.reset(), STALE_WORKER_FIRST_AUTHORITY);

    const importedCount = importedGraph.input("count");
    const importedSelection = await workerSignals.linkedAsync({
      source: () => importedCount(),
      computation: (nextCount, previous) => ({
        count: nextCount,
        previous: previous?.value?.count ?? null,
      }),
    });
    assert.deepEqual(importedSelection(), { count: 2, previous: null });

    await importedSelection.set({ count: 9, previous: "manual" });
    await importedCount.set(7);
    await importedSelection.relink();
    assert.deepEqual(importedSelection(), { count: 7, previous: 9 });

    const replacement = compatibilitySignals.graph("workerFirstLinkedAsyncReplacement", {
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

    assert.throws(() => importedSelection(), STALE_WORKER_FIRST_AUTHORITY);
    await assert.rejects(importedSelection.relink(), STALE_WORKER_FIRST_AUTHORITY);

    await assert.rejects(
      workerSignals.linkedAsync(() => {
        void importedCount.set(8);
        return importedCount();
      }),
      /callback computed authoring cannot mutate signals or transactions while the callback is being invoked/,
    );

    workerSignals.free();
  } finally {
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
