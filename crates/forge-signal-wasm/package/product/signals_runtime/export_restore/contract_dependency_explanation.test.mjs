import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphExportImportRuntime } from "../runtime_fixture/graph_export_import_runtime.mjs";

test("The Contract Dependency Explanation Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphExportImportRuntime());
    const firstName = signals.spec.input("firstName", "Ada");
    const status = signals.spec.input("status", "ready");
    const displayLabel = signals.spec.computedCallback(
      "displayLabel",
      () => `${firstName()} (${status()})`,
    );
    const graph = signals.graph("personCard", {
      inputs: {
        firstName,
        status,
      },
      outputs: {
        publicDisplayName: displayLabel,
      },
    });

    assert.deepEqual(
      graph.inspectDiagnostics().dependenciesForOutput("publicDisplayName"),
      {
        graphId: "personCard",
        outputName: "publicDisplayName",
        publishedId: "personCard.publicDisplayName",
        sourceId: "displayLabel",
        publicInputNames: ["firstName", "status"],
        publicInputSourceIds: ["firstName", "status"],
        transitiveSignalIds: [
          "personCard.publicDisplayName",
          "displayLabel",
          "firstName",
          "status",
        ],
      },
    );
    assert.deepEqual(
      graph.inspectHistory().dependenciesForOutput("publicDisplayName"),
      graph.inspectDiagnostics().dependenciesForOutput("publicDisplayName"),
    );
  } finally {
    await cleanup();
  }
});


