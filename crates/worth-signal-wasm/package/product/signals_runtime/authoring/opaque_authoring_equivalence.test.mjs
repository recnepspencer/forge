import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphPublicationRuntime } from "../runtime_fixture/graph_publication_runtime.mjs";
import { materializeGraphDiagnosticsSurface, materializeGraphHistorySurface } from "../runtime_fixture/surface_materialization.mjs";

test("The Opaque Authoring Equivalence Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    function defineCanonicalGraph(signals) {
      const count = signals.input(1, { debugName: "count" });
      const doubled = signals.computed(() => count() * 2, {
        debugName: "doubled",
      });
      const panel = signals.output(
        () => ({
          count: count(),
          doubled: doubled(),
        }),
        { debugName: "panel" },
      );

      return signals.graph("counter", {
        inputs: { count },
        outputs: { doubled, panel },
      });
    }

    function defineCompatibilityGraph(signals) {
      const count = signals.spec.input("count", 1);
      const doubled = signals.spec.computedCallback(
        "doubled",
        () => count() * 2,
      );
      const panel = signals.spec.outputCallback("panel", () => ({
        count: count(),
        doubled: doubled(),
      }));

      return signals.graph("counter", {
        inputs: { count },
        outputs: { doubled, panel },
      });
    }

    const canonicalGraph = defineCanonicalGraph(
      wrapSignals(createGraphPublicationRuntime()),
    );
    const compatibilityGraph = defineCompatibilityGraph(
      wrapSignals(createGraphPublicationRuntime()),
    );

    assert.equal(canonicalGraph.summary().id, compatibilityGraph.summary().id);
    assert.deepEqual(
      canonicalGraph.summary().inputNames,
      compatibilityGraph.summary().inputNames,
    );
    assert.deepEqual(
      canonicalGraph.summary().outputNames,
      compatibilityGraph.summary().outputNames,
    );
    assert.equal(
      canonicalGraph.contract().inputs.count,
      canonicalGraph.inputDescriptors()[0].sourceId,
    );
    assert.equal(
      canonicalGraph.contract().outputs.panel,
      canonicalGraph.descriptors()[1].publishedId,
    );
    assert.deepEqual(
      canonicalGraph
        .inputDescriptors()
        .map(({ inputName, sourceKind, authority }) => ({
          inputName,
          sourceKind,
          authority,
        })),
      compatibilityGraph
        .inputDescriptors()
        .map(({ inputName, sourceKind, authority }) => ({
          inputName,
          sourceKind,
          authority,
        })),
    );
    assert.deepEqual(
      canonicalGraph
        .descriptors()
        .map(({ outputName, sourceKind, publicationKind }) => ({
          outputName,
          sourceKind,
          publicationKind,
        })),
      compatibilityGraph
        .descriptors()
        .map(({ outputName, sourceKind, publicationKind }) => ({
          outputName,
          sourceKind,
          publicationKind,
        })),
    );
    assert.deepEqual(
      canonicalGraph.readInputs(),
      compatibilityGraph.readInputs(),
    );
    assert.deepEqual(
      Object.keys(canonicalGraph.read()),
      Object.keys(compatibilityGraph.read()),
    );
    const canonicalDiagnostics = materializeGraphDiagnosticsSurface(
      canonicalGraph.inspectDiagnostics(),
    );
    const compatibilityDiagnostics = materializeGraphDiagnosticsSurface(
      compatibilityGraph.inspectDiagnostics(),
    );
    assert.deepEqual(
      canonicalDiagnostics.graph.id,
      compatibilityDiagnostics.graph.id,
    );
    assert.deepEqual(
      Object.keys(canonicalDiagnostics.inputs),
      Object.keys(compatibilityDiagnostics.inputs),
    );
    assert.deepEqual(
      Object.keys(canonicalDiagnostics.outputs),
      Object.keys(compatibilityDiagnostics.outputs),
    );
    assert.deepEqual(
      Object.keys(canonicalDiagnostics.dependencies),
      Object.keys(compatibilityDiagnostics.dependencies),
    );
    const canonicalHistory = materializeGraphHistorySurface(
      canonicalGraph.inspectHistory(),
    );
    const compatibilityHistory = materializeGraphHistorySurface(
      compatibilityGraph.inspectHistory(),
    );
    assert.equal(
      canonicalHistory.contract.graph.id,
      compatibilityHistory.contract.graph.id,
    );
    assert.deepEqual(
      Object.keys(canonicalHistory.contract.inputs),
      Object.keys(compatibilityHistory.contract.inputs),
    );
    assert.deepEqual(
      Object.keys(canonicalHistory.contract.outputs),
      Object.keys(compatibilityHistory.contract.outputs),
    );
    assert.deepEqual(
      Object.keys(canonicalGraph.exportCompatibilityDefinition()),
      Object.keys(compatibilityGraph.exportCompatibilityDefinition()),
    );
    assert.deepEqual(
      Object.keys(canonicalGraph.exportCompatibilityDefinition().outputs),
      Object.keys(compatibilityGraph.exportCompatibilityDefinition().outputs),
    );
  } finally {
    await cleanup();
  }
});


