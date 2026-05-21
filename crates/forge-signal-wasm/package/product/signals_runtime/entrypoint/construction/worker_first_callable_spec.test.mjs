import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("default worker-first root exposes explicit spec handles over the active imported graph", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });

  const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const wizard = compatibilitySignals.scope("wizard");
  const count = wizard.spec.input("count", 2);
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
  const panel = wizard.spec.output("panel", {
    reads: [doubled.id],
    expr: { kind: "read", id: doubled.id },
    identity: { kind: "exact" },
  });
  const graph = compatibilitySignals.graph("workerFirstExplicitSpec", {
    inputs: { count },
    outputs: { doubled, panel },
  });
  const definition = graph.exportDefinition();
  const snapshot = graph.exportSnapshot();

  try {
    const workerSignals = await createSignals();
    const importedGraph = workerSignals.importGraph(definition, snapshot);
    await importedGraph.ready();

    const rootCount = workerSignals.spec.input("wizard.count", 2, { producesAspects: [1] });
    const scopedCount = workerSignals.scope("wizard").spec.input("count", 2, { producesAspects: [1] });
    const rootDoubled = workerSignals.computedSpec("wizard.doubled", {
      reads: [importedGraph.input("count").id],
      expr: {
        kind: "sum",
        args: [
          { kind: "read", id: importedGraph.input("count").id },
          { kind: "read", id: importedGraph.input("count").id },
        ],
      },
      identity: { kind: "exact" },
    });
    const scopedPanel = workerSignals.scope("wizard").outputSpec("panel", {
      reads: [rootDoubled.id],
      expr: { kind: "read", id: rootDoubled.id },
      identity: { kind: "exact" },
    });
    const callbackDoubled = workerSignals.spec.computedCallback(
      "wizard.callbackDoubled",
      () => rootCount() * 2,
    );
    const callbackPanel = workerSignals.scope("wizard").spec.outputCallback(
      "callbackPanel",
      () => callbackDoubled(),
    );

    assert.equal(rootCount.id, importedGraph.input("count").id);
    assert.equal(scopedCount.id, importedGraph.input("count").id);
    assert.equal(rootDoubled.id, "wizard.doubled");
    assert.equal(scopedPanel.id, "wizard.panel");
    assert.equal(callbackDoubled.id, "wizard.callbackDoubled");
    assert.equal(callbackPanel.id, "wizard.callbackPanel");
    assert.equal(rootCount(), 2);
    assert.equal(rootDoubled(), 4);
    assert.equal(scopedPanel(), 4);
    assert.equal(callbackDoubled(), 4);
    assert.equal(callbackPanel(), 4);

    await rootCount.set(7);
    assert.equal(importedGraph.input("count")(), 7);
    assert.equal(rootDoubled(), 14);
    assert.equal(callbackDoubled(), 14);
    assert.equal(callbackPanel(), 14);

    const controller = workerSignals.controller((surface) => {
      const nested = surface.scope("wizard");
      return {
        inputs: {
          count: nested.publicInput(
            nested.spec.input("count", 2, { producesAspects: [1] }),
            { authority: "readOnly" },
          ),
        },
        outputs: {
          panel: nested.outputSpec("panel", {
            reads: ["wizard.doubled"],
            expr: { kind: "read", id: "wizard.doubled" },
            identity: { kind: "exact" },
          }),
        },
      };
    });

    assert.equal(controller.inputs.count.handle.id, "wizard.count");
    assert.equal(controller.outputs.panel.id, "wizard.panel");

    assert.throws(
      () => workerSignals.spec.input("missing", 0),
      /worker-first signals\.spec\.input/,
    );
    assert.throws(
      () => workerSignals.spec.output("wizard.doubled", { reads: [] }),
      /worker-first signals\.spec\.output/,
    );
    assert.throws(
      () => workerSignals.spec.computedCallback("wizard.doubled", () => 1),
      /cannot reuse canonical id|already uses id|requires an unused signal id|already exists/,
    );

    const nextGraph = compatibilitySignals.graph("workerFirstExplicitSpecReplacement", {
      inputs: { replacement: compatibilitySignals.spec.input("replacement", 1) },
      outputs: {
        replacement: compatibilitySignals.spec.output("replacement.output", {
          reads: ["replacement"],
          expr: { kind: "read", id: "replacement" },
          identity: { kind: "exact" },
        }),
      },
    });
    const nextImportedGraph = workerSignals.importGraph(
      nextGraph.exportDefinition(),
      nextGraph.exportSnapshot(),
    );
    await nextImportedGraph.ready();

    assert.throws(
      () => rootCount(),
      /worker-first signals\.spec\.input/,
    );
    assert.throws(
      () => callbackDoubled(),
      /replaced the worker-owned runtime|currently available|worker-first computed/,
    );
    assert.throws(
      () => callbackPanel(),
      /replaced the worker-owned runtime|currently available|worker-first output/,
    );

    await nextImportedGraph.terminate();
    workerSignals.free();
  } finally {
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
