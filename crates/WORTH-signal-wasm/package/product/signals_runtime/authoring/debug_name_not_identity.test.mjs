import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphPublicationRuntime } from "../runtime_fixture/graph_publication_runtime.mjs";

test("The Debug Name Is Not Identity Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphPublicationRuntime());
    const count = signals.input(1, { debugName: "shared" });
    const countMirror = signals.computed(() => count(), {
      debugName: "shared",
    });
    const graph = signals.graph("counter", {
      inputs: {
        count,
      },
      outputs: {
        countValue: countMirror,
      },
    });

    assert.equal(count.debugName, "shared");
    assert.equal(countMirror.debugName, "shared");
    assert.notEqual(count.id, countMirror.id);
    assert.notEqual(count.id, "shared");
    assert.notEqual(countMirror.id, "shared");
    assert.equal(graph.output("countValue").id, "counter.countValue");
    assert.equal(graph.contract().outputs.countValue, "counter.countValue");
    assert.equal(
      graph.exportCompatibilityDefinition().contract.outputs.countValue,
      "counter.countValue",
    );
  } finally {
    await cleanup();
  }
});


