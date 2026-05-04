import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphExportImportRuntime } from "../runtime_fixture/graph_export_import_runtime.mjs";

test("The Public Boundary Naming Truth Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphExportImportRuntime());
    const name = signals.input("Ada", { debugName: "name" });
    const displayLabel = signals.computed(() => name().toUpperCase(), {
      debugName: "displayLabel",
    });
    const namingGraph = signals.graph("naming", {
      inputs: {
        name,
      },
      outputs: {
        publicDisplayName: displayLabel,
      },
    });

    assert.equal(name.debugName, "name");
    assert.equal(displayLabel.debugName, "displayLabel");
    assert.equal(
      namingGraph.output("publicDisplayName").id,
      "naming.publicDisplayName",
    );
    assert.deepEqual(
      {
        ...namingGraph.contract(),
        inputs: { ...namingGraph.contract().inputs },
        outputs: { ...namingGraph.contract().outputs },
      },
      {
        graph: namingGraph.summary(),
        inputs: {
          name: name.id,
        },
        outputs: {
          publicDisplayName: "naming.publicDisplayName",
        },
        inputDescriptors: namingGraph.inputDescriptors(),
        descriptors: namingGraph.descriptors(),
      },
    );
    assert.deepEqual(namingGraph.descriptors(), [
      {
        outputName: "publicDisplayName",
        sourceId: displayLabel.id,
        sourceKind: "computed",
        publishedId: "naming.publicDisplayName",
        publicationKind: "synthesizedOutput",
      },
    ]);
    assert.equal(
      namingGraph.exportCompatibilityDefinition().contract.outputs
        .publicDisplayName,
      "naming.publicDisplayName",
    );
  } finally {
    await cleanup();
  }
});


