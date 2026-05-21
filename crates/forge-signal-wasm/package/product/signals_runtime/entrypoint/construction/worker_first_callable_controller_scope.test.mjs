import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("default worker-first root admits scoped controller and public-input composition over active imported-graph handles", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });

  const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const count = compatibilitySignals.input(4, { debugName: "count" });
  const graph = compatibilitySignals.graph("workerFirstControllerScope", {
    inputs: { count },
    outputs: {
      doubled: compatibilitySignals.computedSpec("worker:first:controller:scope:doubled", {
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

  try {
    const workerSignals = await createSignals();
    const importedGraph = workerSignals.importGraph(definition, snapshot);
    await importedGraph.ready();

    const boundary = workerSignals.publicInput(importedGraph.input("count"), {
      authority: "imported",
      requiredness: "optional",
    });
    const controller = workerSignals.controller((surface) => {
      const nested = surface.scope("editSession");
      return nested.controller({
        inputs: {
          count: nested.publicInput(importedGraph.input("count"), {
            authority: "readOnly",
          }),
          importedCount: boundary,
        },
        outputs: {
          doubled: importedGraph.output("doubled"),
        },
        internal: {
          rawCount: importedGraph.input("count"),
        },
      });
    });

    assert.equal(boundary.handle.id, importedGraph.input("count").id);
    assert.equal(boundary.authority, "imported");
    assert.equal(boundary.requiredness, "optional");
    assert.equal(controller.inputs.count.authority, "readOnly");
    assert.equal(controller.inputs.importedCount.authority, "imported");
    assert.equal(controller.outputs.doubled.id, importedGraph.output("doubled").id);
    assert.equal(controller.internal.rawCount.id, importedGraph.input("count").id);

    const scoped = workerSignals.scope("workflow");
    assert.equal(typeof scoped.controller, "function");
    assert.equal(typeof scoped.publicInput, "function");
    assert.throws(
      () => scoped.input(1),
      /WorkerFirstCallableSurfaceUnavailable/,
    );

    await importedGraph.terminate();
    workerSignals.free();
  } finally {
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("default worker-first root controller composition rejects outputs in publicInput and foreign active-handle leaks", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });

  const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const count = compatibilitySignals.input(4, { debugName: "count" });
  const graph = compatibilitySignals.graph("workerFirstControllerScopeDenials", {
    inputs: { count },
    outputs: { count },
  });
  const definition = graph.exportDefinition();
  const snapshot = graph.exportSnapshot();

  try {
    const workerSignals = await createSignals();
    const importedGraph = workerSignals.importGraph(definition, snapshot);
    await importedGraph.ready();

    assert.throws(
      () => workerSignals.publicInput(importedGraph.output("count")),
      /expects a worker-first input handle/,
    );
    assert.throws(
      () => workerSignals.controller({
        inputs: {
          count: importedGraph.output("count"),
        },
      }),
      /expects a worker-first input handle/,
    );
    assert.throws(
      () => workerSignals.controller({
        outputs: {
          leakedAuthority: workerSignals.publicInput(importedGraph.input("count")),
        },
      }),
      /controller\.outputs\.`leakedAuthority` cannot use signals\.publicInput/,
    );

    await importedGraph.terminate();
    workerSignals.free();
  } finally {
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
