import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphPublicationRuntime } from "../runtime_fixture/graph_publication_runtime.mjs";

test("The Debug Name Is Not Addressability Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphPublicationRuntime());
    const count = signals.input(1, { debugName: "count" });
    const doubled = signals.computed(() => count() * 2, { debugName: "count" });

    assert.equal(count.debugName, "count");
    assert.equal(doubled.debugName, "count");
    assert.notEqual(count.id, "count");
    assert.notEqual(doubled.id, "count");
    assert.notEqual(count.id, doubled.id);
    assert.notEqual(signals.read("count"), count());
  } finally {
    await cleanup();
  }
});


