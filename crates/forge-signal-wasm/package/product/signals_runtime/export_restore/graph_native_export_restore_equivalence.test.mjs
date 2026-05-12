import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphExportImportRuntime } from "../runtime_fixture/graph_export_import_runtime.mjs";

test("The Graph-Native Export And Restore Equivalence Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const sourceSignals = wrapSignals(createGraphExportImportRuntime());
    const count = sourceSignals.input(2, { debugName: "count" });
    const displayLabel = sourceSignals.computed(() => `Count:${count() * 2}`, {
      debugName: "displayLabel",
    });
    const namingGraph = sourceSignals.graph("naming", {
      inputs: {
        count,
      },
      outputs: {
        publicDisplayName: displayLabel,
      },
    });

    count.set(4);
    const exportedDefinition = namingGraph.exportDefinition();
    const exportedSnapshot = namingGraph.exportSnapshot();
    assert.deepEqual(namingGraph.importPosture(), {
      graphId: "naming",
      exactRestoreMode: "SameRuntimeExact",
      portableImport: "Denied",
      portableImportReason:
        "graph-native import currently requires the exact originating runtime envelope",
      hydrate: "Deferred",
      hydrateReason:
        "graph-native portable hydrate is not yet admitted on this surface",
    });
    assert.deepEqual(
      exportedDefinition.importPosture,
      namingGraph.importPosture(),
    );
    assert.deepEqual(
      exportedSnapshot.importPosture,
      namingGraph.importPosture(),
    );

    const restoredSignals = wrapSignals(createGraphExportImportRuntime());
    const restoredGraph = restoredSignals.importGraph(
      exportedDefinition,
      exportedSnapshot,
    );

    assert.deepEqual({ ...restoredGraph.readInputs() }, { count: 4 });
    assert.deepEqual(
      { ...restoredGraph.read() },
      { publicDisplayName: "Count:8" },
    );
    assert.deepEqual(restoredGraph.contract(), namingGraph.contract());
    assert.deepEqual(
      restoredGraph.importPosture(),
      namingGraph.importPosture(),
    );
    assert.deepEqual(restoredGraph.contractHistory(), {
      graphId: "naming",
      current: namingGraph.contract(),
      baseline: namingGraph.contract(),
      deltas: [
        {
          graphId: "naming",
          previousGraphId: "naming",
          changed: false,
          inputs: {
            added: [],
            removed: [],
            remapped: [],
          },
          outputs: {
            added: [],
            removed: [],
            remapped: [],
          },
          inputDescriptorsChanged: [],
          outputDescriptorsChanged: [],
        },
      ],
      changedSinceBaseline: false,
      restoreMode: "SameRuntimeExact",
      importedFromGraphId: "naming",
    });
    assert.equal(
      restoredGraph.exportCompatibilityDefinition().outputs.publicDisplayName,
      "naming.publicDisplayName",
    );
    assert.deepEqual(
      restoredGraph
        .inspectDiagnostics()
        .dependenciesForOutput("publicDisplayName").publicInputNames,
      ["count"],
    );
    assert.equal(
      restoredGraph.inspectHistory().output("publicDisplayName").replay.id,
      "naming.publicDisplayName",
    );

    assert.throws(
      () =>
        restoredSignals.importGraph(exportedDefinition, {
          ...exportedSnapshot,
          id: "other",
        }),
      /requires matching graph ids/,
    );
    assert.throws(
      () =>
        restoredSignals.importGraph(
          {
            ...exportedDefinition,
            contract: {
              ...exportedDefinition.contract,
              outputs: {
                ...exportedDefinition.contract.outputs,
                publicDisplayName: "naming.other",
              },
            },
          },
          exportedSnapshot,
        ),
      /snapshot\.definition\.contract to match the exported graph definition/,
    );
  } finally {
    await cleanup();
  }
});


