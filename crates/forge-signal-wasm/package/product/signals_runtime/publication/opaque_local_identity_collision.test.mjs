import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphPublicationRuntime } from "../runtime_fixture/graph_publication_runtime.mjs";

test("The Opaque Local Identity Collision Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphPublicationRuntime();
    const signals = wrapSignals(rawSignals);

    function createCounterController(namespace) {
      const count = namespace.input(0, { debugName: "count" });
      const doubled = namespace.computed(() => count() * 2, {
        debugName: "count",
      });
      return { count, doubled };
    }

    const left = createCounterController(signals.scope("leftPanel"));
    const right = createCounterController(signals.scope("rightPanel"));

    assert.equal(left.count.debugName, "count");
    assert.equal(left.doubled.debugName, "count");
    assert.equal(right.count.debugName, "count");
    assert.equal(right.doubled.debugName, "count");
    assert.notEqual(left.count.id, "count");
    assert.notEqual(left.doubled.id, "count");
    assert.notEqual(right.count.id, "count");
    assert.notEqual(right.doubled.id, "count");
    assert.notEqual(left.count.id, left.doubled.id);
    assert.notEqual(right.count.id, right.doubled.id);
    assert.notEqual(left.count.id, right.count.id);
    assert.notEqual(left.doubled.id, right.doubled.id);
  } finally {
    await cleanup();
  }
});


