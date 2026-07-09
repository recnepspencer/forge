import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphPublicationRuntime } from "../runtime_fixture/graph_publication_runtime.mjs";
import { materializeGraphDiagnosticsSurface, materializeGraphHistorySurface } from "../runtime_fixture/surface_materialization.mjs";

test("The Scoped Graph And Manual Scope Equivalence Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    function createScopedGraph() {
      const rawSignals = createGraphPublicationRuntime();
      const signals = wrapSignals(rawSignals);

      function createEditSessionController(namespace) {
        const count = namespace.input(1, { id: "count" });
        const label = namespace.computed(() => `count:${count()}`, {
          id: "label",
        });
        return namespace.controller({
          inputs: { count },
          outputs: { label },
        });
      }

      return signals.graph("itemDetail", (graph) => {
        const controller = createEditSessionController(
          graph.scope("editSession"),
        );
        return graph.expose({
          controllers: [controller],
          outputs: {
            count: controller.inputs.count,
          },
        });
      });
    }

    function createManualGraph() {
      const rawSignals = createGraphPublicationRuntime();
      const signals = wrapSignals(rawSignals);
      const count = signals.spec.input("itemDetail.editSession.count", 1);
      const label = signals.spec.computedCallback(
        "itemDetail.editSession.label",
        () => `count:${count()}`,
      );
      return signals.graph("itemDetail", {
        inputs: {
          count,
        },
        outputs: {
          label,
          count,
        },
      });
    }

    const scopedGraph = createScopedGraph();
    const manualGraph = createManualGraph();

    assert.deepEqual(scopedGraph.read(), manualGraph.read());
    assert.deepEqual(scopedGraph.readInputs(), manualGraph.readInputs());
    assert.deepEqual(scopedGraph.summary(), manualGraph.summary());
    assert.deepEqual(scopedGraph.contract(), manualGraph.contract());
    assert.deepEqual(
      scopedGraph.inputDescriptors(),
      manualGraph.inputDescriptors(),
    );
    assert.deepEqual(scopedGraph.descriptors(), manualGraph.descriptors());
    assert.deepEqual(
      materializeGraphDiagnosticsSurface(scopedGraph.inspectDiagnostics()),
      materializeGraphDiagnosticsSurface(manualGraph.inspectDiagnostics()),
    );
    assert.deepEqual(
      materializeGraphHistorySurface(scopedGraph.inspectHistory()),
      materializeGraphHistorySurface(manualGraph.inspectHistory()),
    );
    assert.deepEqual(
      scopedGraph.exportCompatibilityDefinition(),
      manualGraph.exportCompatibilityDefinition(),
    );
  } finally {
    await cleanup();
  }
});


