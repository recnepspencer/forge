import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphExportImportRuntime } from "../runtime_fixture/graph_export_import_runtime.mjs";

test("The Contract Delta And History Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const firstSignals = wrapSignals(createGraphExportImportRuntime());
    const firstName = firstSignals.spec.input("name", "Ada");
    const displayLabel = firstSignals.spec.computedCallback(
      "displayLabel",
      () => firstName().toUpperCase(),
    );
    const graphV1 = firstSignals.graph("naming", {
      inputs: {
        name: firstName,
      },
      outputs: {
        publicDisplayName: displayLabel,
      },
    });

    const secondSignals = wrapSignals(createGraphExportImportRuntime());
    const secondName = secondSignals.spec.input("name", "Ada");
    const displayNameV2 = secondSignals.spec.computedCallback(
      "displayNameV2",
      () => `Person:${secondName()}`,
    );
    const graphV2 = secondSignals.graph("naming", {
      inputs: {
        name: secondName,
      },
      outputs: {
        publicDisplayName: displayNameV2,
      },
    });

    assert.deepEqual(graphV2.contractDelta(graphV1.contract()), {
      graphId: "naming",
      previousGraphId: "naming",
      changed: true,
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
      outputDescriptorsChanged: [
        {
          outputName: "publicDisplayName",
          previousSourceId: "displayLabel",
          currentSourceId: "displayNameV2",
          previousPublishedId: "naming.publicDisplayName",
          currentPublishedId: "naming.publicDisplayName",
          previousSourceKind: "computed",
          currentSourceKind: "computed",
          previousPublicationKind: "synthesizedOutput",
          currentPublicationKind: "synthesizedOutput",
        },
      ],
    });
    assert.deepEqual(graphV1.contractHistory(), {
      graphId: "naming",
      current: graphV1.contract(),
      baseline: null,
      deltas: [],
      changedSinceBaseline: false,
      restoreMode: "LiveRuntime",
      importedFromGraphId: null,
    });
  } finally {
    await cleanup();
  }
});


