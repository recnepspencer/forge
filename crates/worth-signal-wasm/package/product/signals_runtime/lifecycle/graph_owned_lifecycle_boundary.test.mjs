import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphPublicationRuntime } from "../runtime_fixture/graph_publication_runtime.mjs";

test("The Graph-Owned Lifecycle Boundary Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphPublicationRuntime();
    const signals = wrapSignals(rawSignals);

    const ambientCount = signals.spec.input("ambient.count", 1);

    assert.throws(
      () =>
        signals.graph("itemDetail", () => ({
          outputs: {
            count: ambientCount,
          },
        })),
      /must return the result of graph\.expose/,
    );

    assert.throws(
      () =>
        signals.graph("itemDetail", (graph) =>
          graph.expose({
            outputs: {
              count: ambientCount,
            },
          }),
        ),
      /must come from graph-owned scope `itemDetail`/,
    );

    assert.throws(
      () =>
        signals.graph("itemDetail", (graph) => {
          const edit = signals.scope("itemDetail.editSession");
          const count = edit.input(1, { id: "count" });
          return graph.expose({
            inputs: {
              count,
            },
            outputs: {
              count,
            },
          });
        }),
      /must come from graph-owned scope `itemDetail`/,
    );
  } finally {
    await cleanup();
  }
});


